mod concurrent_bag;
mod fs_filter;
mod fs_state;

use fs_filter::*;
use fs_state::*;

use futures;
use futures::StreamExt;
use futures_core;
use futures_core::stream::BoxStream;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tonic::transport::server::TcpIncoming;
use tonic::{Response, Status};
use tracing::{event, info_span, instrument, Level};
use tracing_subscriber;
use tracing_subscriber::{prelude::*, EnvFilter};

mod searchium {
    include!("gen/searchium.v2.rs");
}

type TonicResult<T> = Result<T, tonic::Status>;

// Commands for index state to update
#[derive(Debug)]
enum Command {
    RegisterFolder(
        searchium::FolderRegisterRequest,
        broadcast::Sender<TonicResult<searchium::IndexUpdate>>,
    ),
    UnregisterFolder(searchium::FolderUnregisterRequest),
}

struct Service {
    command_tx: mpsc::Sender<Command>,
}

struct IndexState {
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
            Command::RegisterFolder(params, tx) => {
                let span = info_span!("RegisterFolder");
                let _ = span.enter();
                let timestamp = Some(prost_types::Timestamp::from(SystemTime::now()));
                tx.send(Ok(searchium::IndexUpdate {
                    timestamp,
                    r#type: Some(searchium::index_update::Type::FilesystemScanStart(searchium::GenericEvent{})),
                }))
                .ok();
                event!(Level::DEBUG, ?params.ignore_file_globs, ?params.path, "Constructing path filter");
                let filter =
                    PathGlobFilter::new(PathBuf::from(&params.path), params.ignore_file_globs);
                // TODO: Dupe detection, check for different ignore globs
                let new_root = scan_filesystem(Path::new(&params.path), &filter);
                self.roots.push(new_root);
                let timestamp = Some(prost_types::Timestamp::from(SystemTime::now()));
                tx.send(Ok(searchium::IndexUpdate {
                    timestamp,
                    r#type: Some(searchium::index_update::Type::FilesystemScanEnd(
                        searchium::GenericEvent {},
                    )),
                }))
                .ok();
            }
            Command::UnregisterFolder(_params) => {
                // TODO: Remove root directory if it exists
            }
        }
    }
}

impl Service {
    fn new() -> Self {
        let (command_tx, command_rx) = mpsc::channel(32);
        let state = IndexState {
            command_rx,
            roots: Vec::new(),
        };
        tokio::spawn(async move { state.run().await });
        Service { command_tx }
    }
}
impl std::fmt::Debug for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}

#[tonic::async_trait]
impl searchium::searchium_service_server::SearchiumService for Service {
    #[instrument(err)]
    async fn hello(
        &self,
        request: tonic::Request<searchium::HelloRequest>,
    ) -> TonicResult<Response<searchium::HelloResponse>> {
        println!("Got hello from {}", request.into_inner().id);
        let reply = searchium::HelloResponse {
            id: "rust searchium server".to_string(),
        };
        Ok(Response::new(reply))
    }

    type RegisterFolderStream = BoxStream<'static, TonicResult<searchium::IndexUpdate>>;

    #[instrument(err)]
    async fn register_folder(
        &self,
        request: tonic::Request<searchium::FolderRegisterRequest>,
    ) -> TonicResult<Response<Self::RegisterFolderStream>> {
        let (tx, rx) = broadcast::channel(16);
        match self
            .command_tx
            .send(Command::RegisterFolder(request.into_inner(), tx))
            .await
        {
            Err(e) => Err(Status::internal(format!("{:?}", e))),
            Ok(_) => Ok(Response::new(Box::pin(
                tokio_stream::wrappers::BroadcastStream::from(rx).filter_map(|r| async move {
                    match r {
                        Err(BroadcastStreamRecvError::Lagged(_)) => None,
                        Ok(e) => Some(e),
                    }
                }),
            ))),
        }
    }

    #[instrument(err)]
    async fn unregister_folder(
        &self,
        request: tonic::Request<searchium::FolderUnregisterRequest>,
    ) -> TonicResult<Response<searchium::GenericResponse>> {
        match self
            .command_tx
            .send(Command::UnregisterFolder(request.into_inner()))
            .await
        {
            Err(e) => Err(Status::internal(format!("{:?}", e))),
            Ok(_) => Ok(Response::new(searchium::GenericResponse {
                error: 0,
                log_message: "".to_owned(),
            })),
        }
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
        .add_service(searchium::searchium_service_server::SearchiumServiceServer::new(service))
        .serve_with_incoming(tinc);

    event!(Level::INFO, "Starting searchium index server");
    server.await?;

    event!(Level::INFO, "Shutting down searchium index server");

    Ok(())
}
