mod fs_state;
mod fs_filter;
mod concurrent_bag;

use fs_state::*;
use fs_filter::*;

use futures;
use futures::Future;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::pin::pin;
use std::task::Poll;
use tokio::sync::{broadcast, mpsc};
use tonic::transport::server::TcpIncoming;
use tonic::{Response, Status};
use tracing::{event, info_span, instrument, Level};
use tracing_subscriber;
use tracing_subscriber::{prelude::*, EnvFilter};

include!("gen/searchium.v2.rs");

type TonicResult<T> = Result<T, tonic::Status>;

#[allow(dead_code)]
#[derive(Clone)]
enum Event {
    None,
    IndexProgress(IndexProgressUpdate),
}

// Commands for index state to update
#[derive(Debug)]
enum Command {
    RegisterFolder(FolderRegisterRequest),
    UnregisterFolder(FolderUnregisterRequest),
}

struct Service {
    event_rx: broadcast::Receiver<Event>,
    command_tx: mpsc::Sender<Command>,
}

struct IndexState {
    _event_tx: broadcast::Sender<Event>,
    command_rx: mpsc::Receiver<Command>,
    roots: Vec<Directory>,
}

impl std::fmt::Debug for IndexState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}
// TODO: Setup directory watcher to feed update commands back into index state
impl IndexState {
    async fn run(mut self) {
        event!(Level::INFO, "Starting index state command loop");
        while let Some(c) = self.command_rx.recv().await {
            // TODO: Consider how to handle parallelism here i.e. getting more commands while working on an indexing operation
            self.execute_command(c).await;
        }
        event!(Level::INFO, "Leaving index state command loop");
    }

    async fn execute_command(&mut self, c: Command) {
        match c {
            Command::RegisterFolder(params) => {
                info_span!("RegisterFolder").in_scope(|| {
                    event!(Level::DEBUG, ?params.ignore_file_globs, ?params.path, "Constructing path filter");
                    let filter = PathGlobFilter::new(PathBuf::from(&params.path), params.ignore_file_globs);
                    // TODO: Dupe detection, check for different ignore globs
                    let new_root = scan_filesystem(
                        Path::new(&params.path),
                        &filter
                    );
                    self.roots.push(new_root);
                });
            }
            Command::UnregisterFolder(_params) => {
                // TODO: Remove root directory if it exists
            }
        }
    }
}

impl Service {
    fn new() -> Self {
        let (_event_tx, event_rx) = broadcast::channel::<Event>(32);
        let (command_tx, command_rx) = mpsc::channel(32);
        let state = IndexState {
            _event_tx,
            command_rx,
            roots: Vec::new(),
        };
        tokio::spawn(async move { state.run().await });
        Service {
            event_rx,
            command_tx,
        }
    }
}

struct EventStream<T> {
    event_rx: broadcast::Receiver<Event>,
    filter: fn(Event) -> Option<T>,
}

impl<T> EventStream<T> {
    fn new(event_rx: broadcast::Receiver<Event>, filter: fn(Event) -> Option<T>) -> Self {
        Self { event_rx, filter }
    }
}

impl<T> futures::Stream for EventStream<T> {
    type Item = TonicResult<T>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        loop {
            let event = {
                let next = pin!(self.event_rx.recv());
                let r = match next.poll(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(r) => r,
                };
                let event = match r {
                    Err(e) => return Poll::Ready(Some(Err(Status::internal(format!("{:?}", e))))),
                    Ok(event) => event,
                };
                event
            };
            if let Some(t) = (self.filter)(event) {
                return Poll::Ready(Some(Ok(t)));
            }
        }
    }
}

fn event_to_index_progress(e: Event) -> Option<IndexProgressUpdate> {
    match e {
        Event::IndexProgress(e) => Some(e),
        _ => None,
    }
}

impl std::fmt::Debug for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}

#[tonic::async_trait]
impl searchium_service_server::SearchiumService for Service {
    #[instrument(err)]
    async fn hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> TonicResult<Response<HelloResponse>> {
        println!("Got hello from {}", request.into_inner().id);
        let reply = HelloResponse {
            id: "rust searchium server".to_string(),
        };
        Ok(Response::new(reply))
    }

    #[instrument(err)]
    async fn register_folder(
        &self,
        request: tonic::Request<FolderRegisterRequest>,
    ) -> TonicResult<Response<GenericResponse>> {
        match self
            .command_tx
            .send(Command::RegisterFolder(request.into_inner()))
            .await
        {
            Err(e) => Err(Status::internal(format!("{:?}", e))),
            Ok(_) => Ok(Response::new(GenericResponse {
                error: 0,
                log_message: "".to_owned(),
            })),
        }
    }

    #[instrument(err)]
    async fn unregister_folder(
        &self,
        request: tonic::Request<FolderUnregisterRequest>,
    ) -> TonicResult<Response<GenericResponse>> {
        match self
            .command_tx
            .send(Command::UnregisterFolder(request.into_inner()))
            .await
        {
            Err(e) => Err(Status::internal(format!("{:?}", e))),
            Ok(_) => Ok(Response::new(GenericResponse {
                error: 0,
                log_message: "".to_owned(),
            })),
        }
    }

    type GetIndexProgressStream = EventStream<IndexProgressUpdate>;

    #[instrument(err)]
    async fn get_index_progress(
        &self,
        _request: tonic::Request<EmptyRequest>,
    ) -> TonicResult<Response<Self::GetIndexProgressStream>> {
        let event_rx = self.event_rx.resubscribe();
        let stream = EventStream::<IndexProgressUpdate>::new(event_rx, event_to_index_progress);

        Ok(Response::new(stream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut port = 50050;
    let (tinc, addr) = loop {
        let addr = format!("[::1]:{}", port).parse().unwrap();
        match TcpIncoming::new(addr, true, None) {
            Ok(t) => break (t, addr),
            Err(_) => port += 1,
        }
    };
    // Print address on stdout for client to connect
    println!("{}", addr);

    let fmt = tracing_subscriber::fmt::layer()
        .compact()
        .with_file(false)
        .with_line_number(false);
    tracing_subscriber::registry()
        .with(fmt)
        .with(EnvFilter::from_env("SEARCHIUM_LOG"))
        .init();
    // tracing::subscriber::set_global_default(subscriber)?;

    event!(Level::INFO, "Building searchium index server");

    let service = Service::new();
    let server = tonic::transport::Server::builder()
        .add_service(searchium_service_server::SearchiumServiceServer::new(
            service,
        ))
        .serve_with_incoming(tinc);

    event!(Level::INFO, "Starting searchium index server");
    server.await?;

    event!(Level::INFO, "Shutting down searchium index server");

    Ok(())
}
