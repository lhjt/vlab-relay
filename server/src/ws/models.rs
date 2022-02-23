use tokio_tungstenite::tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};
use tracing::info;

use super::TransmissionChannel;
use crate::relay::ws_extensions::SocketFrame;

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

    /// Send's a message to the peer.
    pub(crate) fn send_message(
        &self,
        message: Message,
    ) -> Result<(), futures::channel::mpsc::TrySendError<Message>> {
        self.channel.unbounded_send(message)
    }

    pub(crate) fn send_socket_frame(&self, frame: &SocketFrame) {
        let encoded = prost::Message::encode_to_vec(frame);
        let message = Message::Binary(encoded);
        self.send_message(message).unwrap();
    }

    /// Closes the peer's connection, with close code `Policy`.
    pub(crate) fn close_with_policy(&self) {
        self.send_message(Message::Close(Some(CloseFrame {
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
