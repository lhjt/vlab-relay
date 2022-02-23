use std::net::SocketAddr;

use prost::bytes::Bytes;
use tokio_tungstenite::tungstenite::Message as WSMessage;
use tracing::{info, instrument, warn};

use super::PeerMap;
use crate::{
    client_manager::tasks::CoreMessage,
    relay::ws_extensions::{task::Data, SocketFrame},
    MANAGER,
};

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

            // message should be binary
            if let WSMessage::Binary(binary) = msg {
                // attempt to decode the message into a SocketFrame
                if let Ok(frame) = <SocketFrame as prost::Message>::decode(Bytes::from(binary)) {
                    if frame.task.is_none() || frame.task.as_ref().unwrap().data.is_none() {
                        // invalid message, no task
                        warn!(
                            "[ws] received invalid message from peer user: `{}`",
                            pd.username
                        );
                        return;
                    }

                    let task = frame.task.unwrap();
                    let task_data = task.data.unwrap();

                    let manager = MANAGER.get().unwrap();
                    // we can ignore opcode since rust processes enum variants :sunglasses:
                    // TODO: validate opcode
                    match task_data {
                        Data::StyleResp(csr) => {
                            manager
                                .tasks
                                .complete_task(task.id, CoreMessage::CheckStyleResponse(csr))
                                .await;
                        },
                        _ => todo!(),
                    }
                } else {
                    warn!(
                        "[ws] failed to decode protobuf from peer user: `{}`",
                        pd.username
                    );
                }
            } else {
                warn!(
                    "[ws] received non-binary message from peer user: `{}`",
                    pd.username
                );
            }
        },
        None => operations::handle_registration(peer, msg, address),
    }
}
