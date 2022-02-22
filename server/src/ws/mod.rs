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
use tracing::{debug, info};

mod messaging;

pub(crate) type Tx = UnboundedSender<Message>;
pub(crate) type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

pub(crate) async fn handle_connection(peer_map: PeerMap, stream: TcpStream, address: SocketAddr) {
    info!("[ws] new connection from peer: {}", address);

    // perform websocket handshake
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("failed to accept websocket stream");
    info!("[ws] websocket connection established: {}", address);

    // register peer
    let (tx, rx) = unbounded();
    peer_map.lock().await.insert(address, tx);

    // channel to send messages to and channel to receive messages from
    let (outgoing, incoming) = ws_stream.split();

    // execute the following closure until the stream closes
    let broadcast_incoming = incoming.try_for_each(|msg| {
        debug!("[ws] received message from peer: {:#}", msg);

        let peer_map = Arc::clone(&peer_map);

        tokio::spawn(async move {
            messaging::handle_message(peer_map, msg, address).await;
        });

        future::ok(())
    });

    // message -> tx:->rx -> outgoing

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    info!("[ws] connection closed: {}", address);
    peer_map.lock().await.remove(&address);
}
