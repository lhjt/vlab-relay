use std::net::SocketAddr;

use prost::{bytes::Bytes, Message};
use tokio_tungstenite::tungstenite::Message as WSMessage;
use tracing::{info, instrument, warn};

use super::{PeerData, PeerMap};
use crate::relay::ws_extensions::InitFrame;

/// Handle a message from a peer. This is called for each message received from
/// each peer.
#[instrument(skip(peer_map))]
pub(crate) async fn handle_message(peer_map: PeerMap, msg: WSMessage, address: SocketAddr) {
    // every peer must register themselves with the server before anything else can
    // take place.

    let mut peer_map = peer_map.lock().await;
    let peer = peer_map
        .get_mut(&address)
        .expect("peer was not in the peer map");

    // TODO: handle initial client registering
    // check if the peer is registered
    match &peer.data {
        Some(pd) => {
            // peer is registered, handle the message
            info!("[ws] received message from peer user: `{}`", pd.username);
        },
        None => {
            // attempt to register the peer
            // check the message type
            if let WSMessage::Binary(data) = msg {
                // attempt to decode it into an InitFrame
                if let Ok(frame) = InitFrame::decode(Bytes::from(data)) {
                    // TODO: check if the frame is valid (i.e. validate code with database and
                    // all that)
                    // We will assume it's fine for the time being
                    let data = PeerData {
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
                    peer.data = Some(PeerData {
                        token:    "dummy".to_string(),
                        username: "dummy".to_string(),
                    });
                }
            } else {
                // the peer is not attempting to validate; close the connection
                peer.close_with_policy();
            }
        },
    }
}
