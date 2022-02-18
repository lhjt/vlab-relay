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
        let response = AutoTestSubmissionResponse {
            test_name:      _request.get_ref().test_name.clone(),
            text_exit_code: 1,
            test_output:    "This service has not been implemented.".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn submit_work(
        &self,
        _request: Request<SubmissionRequest>,
    ) -> Result<Response<SubmissionResponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }
}
