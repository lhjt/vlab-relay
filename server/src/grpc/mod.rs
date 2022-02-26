use tonic::{Request, Response, Status};
use tracing::{debug, error, instrument};

use self::interceptors::is_admin;
use crate::{
    auth::User,
    client_manager::ClientManagerError,
    relay::{
        admin::{DeleteUserRequest, GenericResponse, UpsertUserRequest},
        core::{relay_service_server::RelayService, CommandRequest, CommandResponse},
    },
    MANAGER,
    USER_MANAGER,
};

pub(crate) mod interceptors;
#[macro_use]
mod macros;
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
                return unauthenticated!("You must be authenticated to use this service.");
            },
        };

        let mgr = MANAGER.get().unwrap();
        debug!("[grpc] waiting for task to complete");
        let result = mgr.forward_task(&zid, request.into_inner()).await;

        match result {
            Ok(v) => Ok(Response::new(v)),
            Err(e) => {
                error!("failed to forward task: {:?}", e);
                match e {
                    ClientManagerError::NoRunner => Err(Status::unavailable("NoRunner")),
                }
            },
        }
    }

    #[instrument]
    async fn upsert_user(
        &self,
        request: Request<UpsertUserRequest>,
    ) -> Result<Response<GenericResponse>, Status> {
        validate_admin!(request);

        let mgr = USER_MANAGER.get().unwrap();
        let req = request.into_inner();
        match mgr
            .upsert_user(User {
                zid:   req.zid,
                token: req.token,
            })
            .await
        {
            Ok(_) => generic_success!(),
            Err(e) => generic_failed!("failed to upsert user: {:?}", e),
        }
    }

    #[instrument]
    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<GenericResponse>, Status> {
        validate_admin!(request);

        let mgr = USER_MANAGER.get().unwrap();
        let req = request.into_inner();
        match mgr.delete_by_zid(&req.zid).await {
            Ok(_) => generic_success!(),
            Err(e) => generic_failed!("failed to delete user: {:?}", e),
        }
    }
}
