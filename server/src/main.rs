mod concurrent_bag;
mod fs_filter;
mod fs_state;
mod index_server;

use index_server::*;

use futures;
use futures::StreamExt;
use futures_core;
use futures_core::stream::BoxStream;
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tonic::transport::server::TcpIncoming;
use tonic::{Response, Status};
use tracing::{event, instrument, Level};
use tracing_subscriber;
use tracing_subscriber::{prelude::*, EnvFilter};

mod searchium {
    include!("gen/searchium.v2.rs");
}

type TonicResult<T> = Result<T, tonic::Status>;

struct Service {
    command_tx: mpsc::Sender<Command>,
}

impl Service {
    fn new() -> Self {
        let (command_tx, command_rx) = mpsc::channel(32);
        let state = IndexState::new(command_rx) ;
        state.run(); // moves state 
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

    type SearchFilePathsStream = BoxStream<'static, TonicResult<searchium::FilePathSearchResponse>>;

    #[instrument(err)]
    async fn search_file_paths(
        &self,
        request: tonic::Request<tonic::Streaming<searchium::FilePathSearchRequest>>,
    ) -> Result<tonic::Response<Self::SearchFilePathsStream>, tonic::Status> {
        let mut requests = request.into_inner();
        let command_tx = self.command_tx.clone();
        let (results_tx, results_rx) = mpsc::channel(8);
        tokio::spawn(async move { 
            let mut one_result_rx : Option<oneshot::Receiver<searchium::FilePathSearchResponse>> = None;
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
        //     let mut request_rx; 
        //     let first = match requests.next().await {
        //         None => return,
        //         Some(first) => first?,
        //     };

        //     let do_query = |query| async move {
        //         let (result_tx, mut result_rx) = oneshot::channel();
        //         match command_tx
        //             .send(Command::FilePathSearch(first, result_tx))
        //             .await
        //         {
        //             Ok(_) => Ok(result_rx),
        //             Err(_e) => Err(Status::internal("")),
        //         }
        //     };

        //     let mut result_rx = do_query(first).await?;

        //     loop {
        //         tokio::select! {
        //             Some(query) = requests.next() => {
        //                 match query {
        //                     Ok(query) => {
        //                         match do_query(query).await {
        //                            Ok(rx) => { result_rx = rx; Ok(()) }
        //                            Err(_e) => Err(Status::internal(""))
        //                         }
        //                     }
        //                     Err(_e) => Err(Status::internal(""))
        //                 }
        //             },
        //             result = &mut result_rx => {
        //                 match result {
        //                     Ok(r) => { yield Ok::<_, tonic::Status>(r); Ok(()) },
        //                     Err(_e) => Err(Status::internal(""))
        //                 }
        //             }
        //         }?;
        //     }
        // });
        let stream = tokio_stream::wrappers::ReceiverStream::from(results_rx);
        Ok(Response::new(
            Box::pin(stream) as Self::SearchFilePathsStream
        ))
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
