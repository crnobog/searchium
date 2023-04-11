use tonic::transport::server::TcpIncoming;
use tonic::Response;
use tokio_stream;

include!("gen/searchium.v2.rs");

type TonicResult<T> = Result<T, tonic::Status>;

#[derive(Default)]
struct Service {}

#[tonic::async_trait]
impl searchium_service_server::SearchiumService for Service {
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

    async fn register_folder(
        &self,
        _request: tonic::Request<FolderRegisterRequest>,
    ) -> TonicResult<Response<GenericResponse>> {
        unimplemented!("register_folder");
    }

    async fn unregister_folder(
        &self,
        _request: tonic::Request<FolderUnregisterRequest>,
    ) -> TonicResult<Response<GenericResponse>> {
        unimplemented!("unregister_folder");
    }

    type GetIndexProgressStream = tokio_stream::wrappers::ReceiverStream<TonicResult<IndexProgressUpdate>>;

    async fn get_index_progress(
        &self,
        _request: tonic::Request<EmptyRequest>,
    ) -> TonicResult<Response<Self::GetIndexProgressStream>>
    {   
        unimplemented!("get_index_progress");
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
    println!("{}", addr);
    let service = Service::default();
    tonic::transport::Server::builder()
        .add_service(searchium_service_server::SearchiumServiceServer::new(
            service,
        ))
        .serve_with_incoming(tinc)
        .await?;

    Ok(())
}
