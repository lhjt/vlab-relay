use std::net::SocketAddr;

use prost::bytes::Bytes;
use tokio_tungstenite::tungstenite::Message;
use tracing::{info, warn};

use crate::{
    relay::ws_extensions::InitFrame,
    ws::{models, models::Peer},
};

/// Handle a registration message from a peer.
pub(crate) fn handle_registration(peer: &mut Peer, msg: Message, address: SocketAddr) {
    // attempt to register the peer
    // check the message type
    if let Message::Binary(data) = msg {
        // attempt to decode it into an InitFrame
        if let Ok(frame) = <InitFrame as prost::Message>::decode(Bytes::from(data)) {
            // TODO: check if the frame is valid (i.e. validate code with database and
            // all that)
            // We will assume it's fine for the time being
            let data = models::PeerData {
                token:    frame.token,
                username: frame.zid,
            };

            if data.token.is_empty() {
                warn!("[ws] peer attempted to register with invalid token");
                peer.close_with_policy();
                return;
            }

            // store the data
            info!("[ws] peer `{}`", data.username);
            peer.data = Some(data);
        } else {
            warn!("[ws] failed to decode init frame from peer: {}", address);
            // TODO: we should be closing with policy, but for testing
            // purposes we will auth the client
            // peer.close_with_policy()
            peer.data = Some(models::PeerData {
                token:    "dummy".to_string(),
                username: "dummy".to_string(),
            });
        }
    } else {
        // the peer is not attempting to validate; close the connection
        peer.close_with_policy();
    }
}
