#![warn(clippy::pedantic)]
use std::{collections::HashMap, sync::Arc};

use once_cell::sync::OnceCell;
use tokio::{net::TcpListener, sync::Mutex};
use tonic::transport::Server;
use tracing::{error, info};

use crate::{
    client_manager::ClientManager,
    grpc::{interceptors, Relay},
    relay::core::relay_service_server::RelayServiceServer,
    ws::PeerMap,
};

mod client_manager;
mod grpc;
mod ws;

pub mod relay;

static MANAGER: OnceCell<ClientManager> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let peer_map: PeerMap = Arc::new(Mutex::new(HashMap::new()));
    let manager = ClientManager::new();
    MANAGER.set(manager).unwrap();

    // Launch the websocket server
    let ws_handle = tokio::spawn(async move {
        let listener = TcpListener::bind("0.0.0.0:50052")
            .await
            .expect("failed to bind");

        info!("[ws] listening on {}", "0.0.0.0:50052");

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream
                .peer_addr()
                .expect("connected streams should have a peer address");
            // deal with the connection
            tokio::spawn(ws::handle_connection(peer_map.clone(), stream, peer));
        }
    });

    // Launch the gRPC server
    let rpc_handle = tokio::spawn(async move {
        let grpc_addr = "0.0.0.0:50051".parse().expect("failed to parse address");
        let relay = Relay::default();
        let svc = RelayServiceServer::with_interceptor(relay, interceptors::auth);

        info!("[gRPC] launching gRPC server on {}", grpc_addr);
        Server::builder()
            .accept_http1(true)
            .add_service(tonic_web::enable(svc))
            .serve(grpc_addr)
            .await
            .expect("failed to serve gRPC");
    });

    match tokio::try_join!(ws_handle, rpc_handle) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("[main] error: {}", e);
            panic!()
        },
    }
}
