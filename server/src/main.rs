use tonic::transport::server::TcpIncoming;

include!("gen/searchium.v2.rs");

#[derive(Default)]
struct Service {}

#[tonic::async_trait]
impl searchium_service_server::SearchiumService for Service {
    async fn hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> Result<tonic::Response<HelloResponse>, tonic::Status> {
        println!("Got hello from {}", request.into_inner().id);
        let reply = HelloResponse {
            id: "rust searchium server".to_string(),
        };
        Ok(tonic::Response::new(reply))
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
