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

pub(crate) mod interceptors;
#[derive(Debug, Default)]
pub struct Relay {}

macro_rules! handle_rpc {
    ($request:expr, $mgr:expr, $task:ident) => {{
        let meta = $request.metadata().clone();
        let zid = meta.get("zid").unwrap().to_str().unwrap();

        let mgr = $mgr.get().unwrap();
        let result = mgr.$task(zid, $request.into_inner()).await;

        match result {
            Ok(v) => Ok(Response::new(v)),
            Err(e) => {
                error!("{:?}", e);
                Err(Status::unavailable(
                    "failed to run tests; please try again later",
                ))
            },
        }
    }};
}

#[tonic::async_trait]
impl RelayService for Relay {
    async fn perform_auto_test(
        &self,
        request: Request<AutoTestSubmissionRequest>,
    ) -> Result<Response<AutoTestSubmissionResponse>, Status> {
        handle_rpc!(request, MANAGER, autotest)
    }

    async fn submit_work(
        &self,
        request: Request<SubmissionRequest>,
    ) -> Result<Response<SubmissionResponse>, Status> {
        handle_rpc!(request, MANAGER, submission)
    }

    async fn check_style(
        &self,
        request: Request<CheckStyleRequest>,
    ) -> Result<Response<CheckStyleResponse>, Status> {
        handle_rpc!(request, MANAGER, check_style)
    }
}
