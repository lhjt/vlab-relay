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

use crate::relay;

pub type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

pub async fn handle_connection(peer_map: PeerMap, stream: TcpStream, address: SocketAddr) {
    info!("[ws] new connection from peer: {}", address);

    // perform websocket handshake
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("failed to accept websocket stream");
    info!("[ws] websocket connection established: {}", address);

    // register peer
    let (tx, rx) = unbounded();
    peer_map.lock().await.insert(address, tx);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        debug!(
            "[ws] received message from peer: {}",
            msg.to_text().unwrap()
        );

        let peer_map = Arc::clone(&peer_map);

        tokio::spawn(async move {
            let mut peers = peer_map.lock().await;
            for (_, tx) in peers.iter_mut() {
                let m = relay::AutoTestSubmissionRequest {
                    course_code:   "COMP".to_string(),
                    test_name:     msg.to_text().unwrap().to_string(),
                    code_segments: vec![relay::CodeSegment {
                        file_name: "main.c".to_string(),
                        data:      vec![],
                    }],
                    main_file:     "main.c".to_string(),
                };

                let v = prost::Message::encode_to_vec(&m);
                let msg = Message::Binary(v);

                if let Err(e) = tx.unbounded_send(msg) {
                    debug!("[ws] failed to send message to peer: {}", e);
                }
            }
        });

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    info!("[ws] connection closed: {}", address);
    peer_map.lock().await.remove(&address);
}