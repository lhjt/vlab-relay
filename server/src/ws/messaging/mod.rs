use std::net::SocketAddr;

use tokio_tungstenite::tungstenite::Message as WSMessage;
use tracing::{info, instrument};

use super::PeerMap;

mod operations;

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
        None => operations::handle_registration(peer, msg, address),
    }
}
