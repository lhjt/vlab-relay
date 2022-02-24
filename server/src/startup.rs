use tokio::net::TcpListener;
use tonic::transport::Server;
use tracing::info;

use crate::{grpc::Relay, relay::core::relay_service_server::RelayServiceServer, ws};

pub(crate) fn launch_grpc_server() -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let grpc_addr = "0.0.0.0:50051".parse().expect("failed to parse address");
        let relay = Relay::default();
        let svc = RelayServiceServer::new(relay).accept_gzip().send_gzip();

        info!("[gRPC] launching gRPC server on {}", grpc_addr);
        Server::builder()
            .accept_http1(true)
            .add_service(tonic_web::enable(svc))
            .serve(grpc_addr)
            .await
            .expect("failed to serve gRPC");
    })
}

pub(crate) fn launch_ws_server() -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let listener = TcpListener::bind("0.0.0.0:50052")
            .await
            .expect("failed to bind");

        info!("[ws] listening on {}", "0.0.0.0:50052");

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream
                .peer_addr()
                .expect("connected streams should have a peer address");
            // deal with the connection
            tokio::spawn(ws::handle_connection(stream, peer));
        }
    })
}
