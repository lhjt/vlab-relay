#![warn(clippy::pedantic)]
use tokio::net::TcpListener;
use tonic::transport::Server;
use tracing::info;

use crate::grpc::{relay, Relay};

mod grpc;
mod ws;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Launch the websocket server
    tokio::spawn(async move {
        let listener = TcpListener::bind("0.0.0.0:50052")
            .await
            .expect("failed to bind");

        info!("[ws] listening on {}", "0.0.0.0:50052");

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream
                .peer_addr()
                .expect("connected streams should have a peer address");
            info!("[ws] Peer address: {}", peer);

            // deal with the connection
        }
    });

    // Launch the gRPC server
    let grpc_addr = "[::1]:50051".parse()?;
    let relay = Relay::default();

    info!("[gRPC] launching gRPC server on {}", grpc_addr);
    Server::builder()
        .add_service(relay::relay_service_server::RelayServiceServer::new(relay))
        .serve(grpc_addr)
        .await?;

    Ok(())
}
