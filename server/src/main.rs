mod concurrent_bag;
mod file_contents;
mod fs_filter;
mod fs_state;
mod index_server;
mod search_engine;
mod gen;

use index_server::*;
use gen::*;

use futures::StreamExt;
use futures_core::stream::BoxStream;
use memory_stats::memory_stats;
use tokio_stream::wrappers::BroadcastStream;
use std::fs::File;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tonic::transport::server::TcpIncoming;
use tonic::{Response, Status};
use tracing::{event, instrument, Level};
use tracing_subscriber::{prelude::*, EnvFilter};

type TonicResult<T> = Result<T, tonic::Status>;

struct Service {
    command_tx: mpsc::Sender<Command>,
}

impl Service {
    fn new() -> Self {
        let (command_tx, command_rx) = mpsc::channel(32);
        let state = IndexServer::new(command_rx);
        state.run(); // moves state
        Service { command_tx }
    }
}

// Required for instrumentation macros
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

    async fn set_configuration(
        &self,
        request: tonic::Request<ConfigurationRequest>,
    ) -> TonicResult<tonic::Response<ConfigurationResponse>> {
        self.command_tx
            .send(Command::SetConfiguration(request.into_inner()))
            .await
            .map_err(|_| Status::internal(""))?;
        Ok(Response::new(ConfigurationResponse {}))
    }

    type RegisterFolderStream = BoxStream<'static, TonicResult<IndexUpdate>>;

    #[instrument(err)]
    async fn register_folder(
        &self,
        request: tonic::Request<FolderRegisterRequest>,
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

    type SearchFilePathsStream = BoxStream<'static, TonicResult<FilePathSearchResponse>>;

    #[instrument(err)]
    async fn search_file_paths(
        &self,
        request: tonic::Request<tonic::Streaming<FilePathSearchRequest>>,
    ) -> TonicResult<tonic::Response<Self::SearchFilePathsStream>> {
        let mut requests = request.into_inner();
        let command_tx = self.command_tx.clone();
        let (results_tx, results_rx) = mpsc::channel(8);
        tokio::spawn(async move {
            let mut one_result_rx: Option<oneshot::Receiver<FilePathSearchResponse>> =
                None;
            loop {
                tokio::select! {
                    Some(query) = requests.next() => {
                        let (tx, rx) = oneshot::channel();
                        command_tx.send(Command::FilePathSearch(query.unwrap(), tx)).await.unwrap();
                        one_result_rx = Some(rx);
                    },
                    result = async { one_result_rx.as_mut().unwrap().await }, if one_result_rx.is_some() => {
                        one_result_rx = None;
                        results_tx.send(Ok::<_, Status>(result.unwrap())).await.unwrap();
                    }
                }
            }
            // Ok::<(), tonic::Status>(())
        });
        let stream = tokio_stream::wrappers::ReceiverStream::from(results_rx);
        Ok(Response::new(
            Box::pin(stream) as Self::SearchFilePathsStream
        ))
    }

    async fn search_file_contents(
        &self,
        request: tonic::Request<FileContentsSearchRequest>,
    ) -> TonicResult<tonic::Response<FileContentsSearchResponse>> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(Command::FileContentsSearch(request.into_inner(), tx))
            .await
            .map_err(|_| Status::internal(""))?;
        Ok(Response::new(rx.await.map_err(|_| Status::internal(""))??))
    }

    async fn get_file_extracts(
        &self,
        request: tonic::Request<FileExtractsRequest>,
    ) -> TonicResult<tonic::Response<FileExtractsResponse>> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(Command::GetFileExtracts(request.into_inner(), tx))
            .await
            .map_err(|_| Status::internal(""))?;
        let result = rx.await.map_err(|_| Status::internal(""))??;
        Ok(Response::new(result))
    }

    async fn get_process_info(
        &self,
        _request: tonic::Request<ProcessInfoRequest>,
    ) -> TonicResult<tonic::Response<ProcessInfoResponse>> {
        let stats = memory_stats().ok_or_else(|| Status::internal(""))?;
        Ok(Response::new(ProcessInfoResponse {
            physical_memory: stats.physical_mem as u64,
            virtual_memory: stats.virtual_mem as u64,
        }))
    }

    async fn get_database_details(
        &self,
        _request: tonic::Request<DatabaseDetailsRequest>,
    ) -> TonicResult<tonic::Response<DatabaseDetailsResponse>> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(Command::GetDatabaseDetails(tx))
            .await
            .map_err(|_| Status::internal(""))?;

        Ok(Response::new(rx.await.map_err(|_| Status::internal(""))??))
    }

    type GetStatusStream = BoxStream<'static, TonicResult<StatusResponse>> ;
    async fn get_status(
        &self,
        _request: tonic::Request<StatusRequest>,
    ) -> TonicResult<tonic::Response<Self::GetStatusStream>> {
        let (tx, rx) = oneshot::channel();
        self.command_tx.send(Command::GetStatusStream(tx)).await.map_err(|_| Status::internal(""))?;
        let rx = rx.await.map_err(|_| Status::internal(""))?;
        let stream = BroadcastStream::from(rx);
        let stream = async_stream::try_stream! {
            for await r in stream { 
                yield r.map_err(|_| Status::internal(""))?;
            }
        };
        Ok(Response::new(Box::pin(stream)))
    }
}

fn setup_trace() {
    let fmt = tracing_subscriber::fmt::layer()
        .compact()
        .with_file(false)
        .with_line_number(false);
    let log_file = File::create("searchium.log").ok().map(|f| {
        tracing_subscriber::fmt::layer()
            .compact()
            .with_ansi(false)
            .with_writer(Arc::new(f))
    });
    tracing_subscriber::registry()
        .with(log_file)
        .with(fmt)
        .with(EnvFilter::from_env("SEARCHIUM_LOG"))
        .init();
    // tracing::subscriber::set_global_default(subscriber)?;
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
    setup_trace();

    event!(Level::INFO, "Building searchium index server");

    let service = Service::new();
    let server = tonic::transport::Server::builder()
        .add_service(searchium_service_server::SearchiumServiceServer::new(service))
        .serve_with_incoming(tinc);

    event!(Level::INFO, "Starting searchium index server");
    server.await?;

    event!(Level::INFO, "Shutting down searchium index server");

    Ok(())
}
