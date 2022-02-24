use std::net::SocketAddr;

use tokio_tungstenite::tungstenite::Message;
use tracing::{instrument, warn};

use crate::{
    relay::ws_extensions::{socket_frame::Data, SocketFrame},
    MANAGER,
};

mod operations;

/// Handle a message from a peer. This is called for each message received from
/// each peer.
#[instrument]
pub(crate) async fn handle_message(msg: Message, address: SocketAddr) {
    // every peer must register themselves with the server before anything else can
    // take place.

    let peer_map = MANAGER.get().unwrap().peers.clone();
    let mut peer_map = peer_map.lock().await;
    let peer = peer_map
        .get_mut(&address)
        .expect("peer was not in the peer map");

    if let Message::Binary(binary) = msg {
        // determine if the peer has been registered
        let not_registered = peer.data.is_none();

        // attempt to decode the message
        let message = match <SocketFrame as prost::Message>::decode(binary.as_ref()) {
            Ok(message) if message.data.is_some() => message.data.unwrap(),
            _ => {
                // peers should not be sending malformed requests, so we
                // close the connection with a policy error
                warn!("[ws] error decoding socket frame");
                peer.close_with_policy();
                return;
            },
        };

        // if the peer is not registered, check if the packet is a InitFrame
        if not_registered {
            if let Data::Init(frame) = message {
                operations::handle_registration(peer, frame).await;
            } else {
                // unregistered peers should not be sending non-init frames
                warn!("[ws] unregistered peer sent non-init frame");
                peer.close_with_policy();
            }
        } else {
            // peer is registered; handle message accordingly
            match message {
                Data::Init(_) => {
                    // registered clients should not be sending init frames
                    warn!("[ws] registered peer sent init frame");
                    // so we will close it because sus
                    peer.close_with_policy();
                },
                Data::TaskRequest(_) => {
                    // runners should not be sending task requests to the server
                    warn!("[ws] runner sent task request");
                    peer.close_with_policy();
                },
                Data::TaskResponse(response) => {
                    // signal to the task manager that this task has been completed
                    let manager = MANAGER.get().unwrap();
                    manager.tasks.complete_task(response).await;
                },
            }
        }
    } else {
        // this service does not deal with any other message types since
        // everything should be serialised in protocol buffer wire encoding
        warn!("[ws] received message from peer: {:#}", msg);
        peer.close_with_policy();
    }
}
