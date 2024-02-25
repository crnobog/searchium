mod concurrent_bag;
mod file_contents;
mod fs_filter;
mod fs_state;
mod gen;
mod index;
mod search_engine;
mod service;

use gen::*;

use std::fs::File;
use std::sync::Arc;
use tonic::transport::server::TcpIncoming;
use tracing::{event, Level};
use tracing_subscriber::{prelude::*, EnvFilter};


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

    let service = service::new();
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
