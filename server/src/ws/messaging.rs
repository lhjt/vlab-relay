use std::net::SocketAddr;

use tokio_tungstenite::tungstenite::Message;

use super::PeerMap;

pub(crate) async fn handle_message(peer_map: PeerMap, msg: Message, address: SocketAddr) {
    // send the message back to the peer
    let peer_map = peer_map.lock().await;
    let peer = peer_map
        .iter()
        .find(|(addr, _)| **addr == address)
        .unwrap()
        .1;
    peer.unbounded_send(msg).unwrap();
}
