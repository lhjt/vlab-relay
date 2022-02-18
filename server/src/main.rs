use relay::{
    AutoTestSubmissionRequest,
    AutoTestSubmissionResponse,
    SubmissionRequest,
    SubmissionResponse,
};
use tokio::net::TcpListener;
use tonic::{transport::Server, Request, Response, Status};
use tracing::info;

pub mod relay {
    tonic::include_proto!("core");
}

#[derive(Debug, Default)]
pub struct Relay {}

#[tonic::async_trait]
impl relay::relay_service_server::RelayService for Relay {
    async fn perform_auto_test(
        &self,
        _request: Request<AutoTestSubmissionRequest>,
    ) -> Result<Response<AutoTestSubmissionResponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn submit_work(
        &self,
        _request: Request<SubmissionRequest>,
    ) -> Result<Response<SubmissionResponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // websocket server
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

    let grpc_addr = "[::1]:50051".parse()?;
    let relay = Relay::default();

    info!("[gRPC] launching gRPC server on {}", grpc_addr);
    Server::builder()
        .add_service(relay::relay_service_server::RelayServiceServer::new(relay))
        .serve(grpc_addr)
        .await?;

    Ok(())
}
