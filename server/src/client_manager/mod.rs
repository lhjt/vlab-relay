use std::{collections::HashMap, sync::Arc};

use snafu::Snafu;
use tokio::sync::RwLock;
use tracing::{error, instrument};

use self::tasks::TaskList;
use crate::{
    relay::{
        core::{CommandRequest, CommandResponse},
        ws_extensions::{socket_frame::Data, SocketFrame, TaskRequest},
    },
    ws::PeerMap,
};

/// The manager that contains all peer data, handles message routing, and
/// executes orders against connected runners.
#[derive(Debug, Clone)]
pub(crate) struct ClientManager {
    /// The map of peers.
    pub(crate) peers: PeerMap,
    /// The list of tasks.
    pub(crate) tasks: TaskList,
}

pub(crate) mod tasks;

#[derive(Debug, Snafu)]
pub(crate) enum ClientManagerError {
    #[snafu(display("no active runner connected"))]
    NoRunner,
}

macro_rules! get_peer_by_zid {
    ($zid:expr, $peers:expr) => {
        $peers
            .iter()
            .find(|(_, peer)| match peer.data.as_ref() {
                Some(data) => data.username == $zid,
                None => false,
            })
            .ok_or(ClientManagerError::NoRunner)?
    };
}

impl ClientManager {
    pub(crate) fn new() -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            tasks: TaskList::new(),
        }
    }

    #[instrument]
    pub(crate) async fn forward_task(
        &self,
        zid: &str,
        task: CommandRequest,
    ) -> Result<CommandResponse, ClientManagerError> {
        // first find the specific peer to send the message to
        let peer_map = self.peers.read().await;
        let peer = get_peer_by_zid!(zid, peer_map);

        // spawn a new oneshot channel for receiving the response
        let (tx, rx) = tokio::sync::oneshot::channel::<Option<CommandResponse>>();

        // create a new task and add it to the list
        let task_id = uuid::Uuid::new_v4().to_string();
        self.tasks.add_task(task_id.clone(), tx).await;

        let send_frame = SocketFrame {
            data: Some(Data::TaskRequest(TaskRequest {
                id:      task_id,
                command: Some(task),
            })),
        };

        // send the task to the peer
        peer.1.send_socket_frame(&send_frame);

        // wait for the response
        let result = rx.await.unwrap();

        // convert the core message into a auto test submission response
        if let Some(result) = result {
            Ok(result)
        } else {
            error!("received unexpected response type");
            panic!();
        }
    }
}
