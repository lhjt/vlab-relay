use relay::{
    AutoTestSubmissionRequest, AutoTestSubmissionResponse, SubmissionRequest, SubmissionResponse,
};
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
        request: Request<AutoTestSubmissionRequest>,
    ) -> Result<Response<AutoTestSubmissionResponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn submit_work(
        &self,
        request: Request<SubmissionRequest>,
    ) -> Result<Response<SubmissionResponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = "[::1]:50051".parse()?;
    let relay = Relay::default();

    info!("[gRPC] launching gRPC server on {}", addr);
    Server::builder()
        .add_service(relay::relay_service_server::RelayServiceServer::new(relay))
        .serve(addr)
        .await?;

    Ok(())
}
