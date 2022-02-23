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
        SubmissionRequest,
        SubmissionResponse,
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

macro_rules! basic_relay {
    ($self: expr, $zid:ident, $req:expr, $req_type:ident, $resp_type:ident) => {{
        // first find the specific peer to send the message to
        let peer_map = $self.peers.lock().await;
        let peer = get_peer_by_zid!($zid, peer_map);

        // spawn a new oneshot channel for receiving the response
        let (tx, rx) = tokio::sync::oneshot::channel::<CoreMessage>();

        let csr = CoreMessage::$req_type($req);

        // create a new task and add it to the list
        let task_id = uuid::Uuid::new_v4().to_string();
        $self.tasks.add_task(task_id.clone(), tx).await;

        // send the task to the peer
        peer.1.send_socket_frame(&csr.into_socket_frame(task_id));

        // wait for the response
        let result = rx.await.unwrap();

        // convert the core message into a auto test submission response
        if let CoreMessage::$resp_type(csr) = result {
            Ok(csr)
        } else {
            error!("received unexpected response type");
            panic!();
        }
    }};
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
        req: CheckStyleRequest,
    ) -> Result<CheckStyleResponse, Error> {
        basic_relay!(self, zid, req, CheckStyleRequest, CheckStyleResponse)
    }

    #[instrument]
    pub(crate) async fn autotest(
        &self,
        zid: &str,
        req: AutoTestSubmissionRequest,
    ) -> Result<AutoTestSubmissionResponse, Error> {
        basic_relay!(
            self,
            zid,
            req,
            AutoTestSubmissionRequest,
            AutoTestSubmissionResponse
        )
    }

    #[instrument]
    pub(crate) async fn submission(
        &self,
        zid: &str,
        req: SubmissionRequest,
    ) -> Result<SubmissionResponse, Error> {
        basic_relay!(self, zid, req, SubmissionRequest, SubmissionResponse)
    }
}
