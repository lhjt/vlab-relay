use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    future,
    pin_mut,
    StreamExt,
    TryStreamExt,
};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, info, instrument, warn};

use self::models::Peer;
use crate::MANAGER;

mod messaging;
pub(crate) mod models;

pub(crate) type TransmissionChannel = UnboundedSender<Message>;
pub(crate) type PeerMap = Arc<Mutex<HashMap<SocketAddr, Peer>>>;

#[instrument(skip(stream))]
pub(crate) async fn handle_connection(stream: TcpStream, address: SocketAddr) {
    info!("[ws] new connection from peer: {}", address);

    // perform websocket handshake
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("failed to accept websocket stream");
    info!("[ws] websocket connection established: {}", address);

    // register peer
    let peer_map = MANAGER.get().unwrap().peers.clone();
    let (tx, rx) = unbounded();
    peer_map.lock().await.insert(address, Peer::new(tx));

    // channel to send messages to and channel to receive messages from
    let (outgoing, incoming) = ws_stream.split();

    // execute the following closure until the stream closes
    let broadcast_incoming = incoming.try_for_each(|msg| {
        debug!("[ws] received message from peer: {:#}", msg);

        // if this is a close message, we will not process it
        if let Message::Close(_) = msg {
            return future::ready(Ok(()));
        }

        tokio::spawn(async move {
            messaging::handle_message(msg, address).await;
        });

        future::ok(())
    });

    // message -> tx:->rx -> outgoing

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);

    // handle registration timeout
    let peers = Arc::clone(&peer_map);
    tokio::spawn(async move {
        // give the client 3 seconds to register itself
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        // if the client hasn't registered itself, close the connection
        let peers = peers.lock().await;
        match peers.get(&address) {
            Some(peer) if peer.data.is_none() => {
                warn!(
                    "[ws] client didn't register itself; closing connection: {}",
                    address
                );
                peer.close_with_policy();
            },
            _ => {},
        }
    });

    future::select(broadcast_incoming, receive_from_others).await;

    info!("[ws] connection closed: {}", address);
    peer_map.lock().await.remove(&address);
}
