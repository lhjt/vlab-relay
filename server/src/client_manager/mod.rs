use std::{collections::HashMap, sync::Arc};

use snafu::Snafu;
use tokio::sync::Mutex;
use tracing::{error, instrument};

use self::tasks::TaskList;
use crate::{
    client_manager::tasks::CoreMessage,
    relay::core::{
        AutoTestSubmissionRequest,
        AutoTestSubmissionResponse,
        CheckStyleRequest,
        CheckStyleResponse,
        CodeSegment,
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
pub(crate) enum Error {
    #[snafu(display("no active runner connected for"))]
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
            .ok_or(Error::NoRunner)?
    };
}

impl ClientManager {
    pub(crate) fn new() -> Self {
        Self {
            peers: Arc::new(Mutex::new(HashMap::new())),
            tasks: TaskList::new(),
        }
    }

    #[instrument]
    pub(crate) async fn check_style(
        &self,
        zid: &str,
        code: &[u8],
    ) -> Result<CheckStyleResponse, Error> {
        // first find the specific peer to send the message to
        let peer_map = self.peers.lock().await;
        let peer = get_peer_by_zid!(zid, peer_map);

        // spawn a new oneshot channel for receiving the response
        let (tx, rx) = tokio::sync::oneshot::channel::<CoreMessage>();

        let csr = CoreMessage::CheckStyleRequest(CheckStyleRequest {
            code_segments: vec![CodeSegment {
                file_name: "t-format.c".to_string(),
                data:      code.to_vec(),
            }],
            main_file:     "t-format.c".to_string(),
        });

        // create a new task and add it to the list
        let task_id = uuid::Uuid::new_v4().to_string();
        self.tasks.add_task(task_id.clone(), tx).await;

        // send the task to the peer
        peer.1.send_socket_frame(&csr.into_socket_frame(task_id));

        // wait for the response
        let result = rx.await.unwrap();

        // convert the core message into a check style response
        if let CoreMessage::CheckStyleResponse(csr) = result {
            Ok(csr)
        } else {
            error!("received a task response that wasn't a check style response");
            panic!();
        }
    }

    #[instrument]
    pub(crate) async fn autotest(
        &self,
        zid: &str,
        req: AutoTestSubmissionRequest,
    ) -> Result<AutoTestSubmissionResponse, Error> {
        // first find the specific peer to send the message to
        let peer_map = self.peers.lock().await;
        let peer = get_peer_by_zid!(zid, peer_map);

        // spawn a new oneshot channel for receiving the response
        let (tx, rx) = tokio::sync::oneshot::channel::<CoreMessage>();

        let csr = CoreMessage::AutoTestSubmissionRequest(req);

        // create a new task and add it to the list
        let task_id = uuid::Uuid::new_v4().to_string();
        self.tasks.add_task(task_id.clone(), tx).await;

        // send the task to the peer
        peer.1.send_socket_frame(&csr.into_socket_frame(task_id));

        // wait for the response
        let result = rx.await.unwrap();

        // convert the core message into a auto test submission response
        if let CoreMessage::AutoTestSubmissionResponse(csr) = result {
            Ok(csr)
        } else {
            error!("received a task response that wasn't a auto test submission response");
            panic!();
        }
    }
}
