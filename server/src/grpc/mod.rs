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
        let zid = match interceptors::get_zid(request.metadata()).await {
            Some(z) => z,
            None => {
                return Err(Status::new(
                    tonic::Code::Unauthenticated,
                    "You must be authenticated to use this service.",
                ));
            },
        };

        let mgr = MANAGER.get().unwrap();
        let result = mgr.forward_task(&zid, request.into_inner()).await;

        match result {
            Ok(v) => Ok(Response::new(v)),
            Err(e) => {
                error!("failed to forward task: {:?}", e);
                Err(Status::unavailable(
                    // TODO: more detailed error messages
                    "failed to forward task; please try again later",
                ))
            },
        }
    }
}
