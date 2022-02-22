use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    future,
    pin_mut,
    StreamExt,
    TryStreamExt,
};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};
use tracing::{debug, info};

mod messaging;

pub(crate) type TransmissionChannel = UnboundedSender<Message>;
pub(crate) type PeerMap = Arc<Mutex<HashMap<SocketAddr, Peer>>>;

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

    /// Closes the peer's connection, with close code `Policy`.
    pub(crate) fn close_with_policy(&self) {
        self.channel
            .unbounded_send(Message::Close(Some(CloseFrame {
                code:   CloseCode::Policy,
                reason: "".into(),
            })))
            .unwrap();
    }
}

/// Data related to a specific peer. This includes identifying information.
pub(crate) struct PeerData {
    pub(crate) token:    String,
    pub(crate) username: String,
}

pub(crate) async fn handle_connection(peer_map: PeerMap, stream: TcpStream, address: SocketAddr) {
    info!("[ws] new connection from peer: {}", address);

    // perform websocket handshake
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("failed to accept websocket stream");
    info!("[ws] websocket connection established: {}", address);

    // register peer
    let (tx, rx) = unbounded();
    peer_map.lock().await.insert(address, Peer::new(tx));

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