use std::net::SocketAddr;

use prost::Message;
use tokio_tungstenite::tungstenite::Message as WSMessage;
use tracing::info;

use super::{PeerData, PeerMap};
use crate::relay::ws_extensions::InitFrame;

/// Handle a message from a peer. This is called for each message received from
/// each peer.
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
                match InitFrame::decode(prost::bytes::Bytes::from(data)) {
                    Ok(frame) => {
                        // TODO: check if the frame is valid (i.e. validate code with database and
                        // all that)
                        // We will assume it's fine for the time being
                        let data = PeerData {
                            token:    frame.token,
                            username: frame.zid,
                        };
                        // store the data
                        peer.data = Some(data);
                    },
                    Err(_) => {
                        // TODO: we should be closing with policy, but for testing
                        // purposes we will auth the client
                        // peer.close_with_policy()
                        peer.data = Some(PeerData {
                            token:    "dummy".to_string(),
                            username: "dummy".to_string(),
                        });
                    },
                }
            } else {
                // the peer is not attempting to validate; close the connection
                peer.close_with_policy();
            }
        },
    }
}
