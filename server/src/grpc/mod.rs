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
        request: Request<AutoTestSubmissionRequest>,
    ) -> Result<Response<AutoTestSubmissionResponse>, Status> {
        let zid = "z5555555";

        let mgr = MANAGER.get().unwrap();
        let result = mgr.autotest(zid, request.into_inner()).await;

        match result {
            Ok(v) => Ok(Response::new(v)),
            Err(e) => {
                error!("{:?}", e);
                Err(Status::unavailable(
                    "failed to run tests; please try again later",
                ))
            },
        }
    }

    async fn submit_work(
        &self,
        request: Request<SubmissionRequest>,
    ) -> Result<Response<SubmissionResponse>, Status> {
        let zid = "z5555555";

        let mgr = MANAGER.get().unwrap();
        let result = mgr.submission(zid, request.into_inner()).await;

        match result {
            Ok(v) => Ok(Response::new(v)),
            Err(e) => {
                error!("{:?}", e);
                Err(Status::unavailable(
                    "failed to submit; please try again later",
                ))
            },
        }
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
