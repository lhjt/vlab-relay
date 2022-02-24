use tonic::{Request, Response, Status};
use tracing::{error, instrument};

use crate::{
    relay::core::{relay_service_server::RelayService, CommandRequest, CommandResponse},
    MANAGER,
};

pub(crate) mod interceptors;
#[derive(Debug, Default)]
pub struct Relay {}

#[tonic::async_trait]
impl RelayService for Relay {
    #[instrument]
    async fn command(
        &self,
        request: Request<CommandRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        let meta = request.metadata().clone();
        let zid = meta.get("zid").unwrap().to_str().unwrap();

        let mgr = MANAGER.get().unwrap();
        let result = mgr.forward_task(zid, request.into_inner()).await;

        match result {
            Ok(v) => Ok(Response::new(v)),
            Err(e) => {
                error!("{:?}", e);
                Err(Status::unavailable(
                    // TODO: more detailed error messages
                    "failed to forward task; please try again later",
                ))
            },
        }
    }
}
