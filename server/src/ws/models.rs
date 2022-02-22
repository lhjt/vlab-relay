use tokio_tungstenite::tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};
use tracing::info;

use super::TransmissionChannel;

/// A websocket peer.
#[derive(Debug, Clone)]
pub(crate) struct Peer {
    pub(crate) channel: TransmissionChannel,
    pub(crate) data:    Option<PeerData>,
}

impl Peer {
    pub(crate) fn new(tx: TransmissionChannel) -> Self {
        Self {
            channel: tx,
            data:    None,
        }
    }

    /// Closes the peer's connection, with close code `Policy`.
    pub(crate) fn close_with_policy(&self) {
        self.channel
            .unbounded_send(Message::Close(Some(CloseFrame {
                code:   CloseCode::Policy,
                reason: "".into(),
            })))
            .unwrap();
        info!("[ws] closing peer connecting with close code `Policy`");
    }
}

/// Data related to a specific peer. This includes identifying information.
#[derive(Debug, Clone)]
pub(crate) struct PeerData {
    pub(crate) token:    String,
    pub(crate) username: String,
}
