use tonic::{Request, Response, Status};

use crate::relay::{
    self,
    AutoTestSubmissionRequest,
    AutoTestSubmissionResponse,
    SubmissionRequest,
    SubmissionResponse,
};
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
