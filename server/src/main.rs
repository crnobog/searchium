include!("gen/searchium.v2.rs");

#[derive(Default)]
struct Service {}

#[tonic::async_trait]
impl searchium_service_server::SearchiumService for Service {
    async fn hello(&self, request : tonic::Request<HelloRequest> ) -> Result<tonic::Response<HelloResponse>, tonic::Status> { 
        println!("Got hello from {}", request.into_inner().id);
        let reply = HelloResponse { id : "rust searchium server".to_string() };
        Ok(tonic::Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let service = Service::default();

    println!("Searchium Server listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(searchium_service_server::SearchiumServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
