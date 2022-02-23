use tonic::{Request, Response, Status};
use tracing::error;

use crate::{
    relay::core::{
        relay_service_server::RelayService,
        AutoTestSubmissionRequest,
        AutoTestSubmissionResponse,
        CheckStyleRequest,
        CheckStyleResponse,
        SubmissionRequest,
        SubmissionResponse,
    },
    MANAGER,
};
#[derive(Debug, Default)]
pub struct Relay {}

#[tonic::async_trait]
impl RelayService for Relay {
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

    async fn check_style(
        &self,
        request: Request<CheckStyleRequest>,
    ) -> Result<Response<CheckStyleResponse>, Status> {
        let zid = "z5555555";
        let code = request.get_ref().code_segments[0].data.clone();

        let mgr = MANAGER.get().unwrap();
        let result = mgr.check_style(zid, &code).await;

        match result {
            Ok(v) => Ok(Response::new(v)),
            Err(e) => {
                error!("{:?}", e);
                Err(Status::unavailable(
                    "failed to check style; please try again later",
                ))
            },
        }
    }
}
