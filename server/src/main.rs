#![warn(clippy::pedantic)]

use auth::UserManager;
use once_cell::sync::OnceCell;
use startup::{launch_grpc_server, launch_ws_server};
use tracing::error;

use crate::client_manager::ClientManager;

mod auth;
mod client_manager;
mod grpc;
mod startup;
mod ws;

pub mod relay;

static MANAGER: OnceCell<ClientManager> = OnceCell::new();
static USER_MANAGER: OnceCell<UserManager> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // set global manager
    MANAGER.set(ClientManager::new()).unwrap();

    // set user manager
    USER_MANAGER.set(UserManager::new().await).unwrap();

    // launch the servers
    let ws_handle = launch_ws_server();
    let rpc_handle = launch_grpc_server();

    match tokio::try_join!(ws_handle, rpc_handle) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("[main] error: {}", e);
            panic!()
        },
    }
}
