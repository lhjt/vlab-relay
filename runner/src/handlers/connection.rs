use std::time::Duration;

use colored::Colorize;
use futures::{channel::mpsc, future, pin_mut, StreamExt, TryStreamExt};
use log::{error, warn};
use spinners::Spinner;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{
    handlers::message::handle_message,
    relay::ws_extensions::{socket_frame::Data, InitFrame, SocketFrame},
};

pub(crate) async fn handle_connection(
    spinner: Spinner,
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    token: String,
) {
    spinner.stop_with_message("✔ Connected to relay \n".green().to_string());

    // create proxy channel to relay messages
    let (tx, rx) = mpsc::unbounded();
    let (write, read) = stream.split();
    let write = rx.map(Ok).forward(write);

    // login to the relay
    let frame = SocketFrame {
        data: Some(Data::Init(InitFrame {
            zid: whoami::username(),
            token,
        })),
    };

    let vec_to_send = prost::Message::encode_to_vec(&frame);
    // send the login frame
    if let Err(e) = tx.unbounded_send(Message::Binary(vec_to_send)) {
        error!("failed to send login frame: {}", e);
        warn!("disconnected from relay; will attempt to reconnect in 5 seconds");
        tokio::time::sleep(Duration::from_secs(5)).await;
        return;
    }

    // execute closure for each message received
    let messages = read.try_for_each(|msg| {
        let tx = tx.clone();

        tokio::spawn(async move {
            // determine the type of msg received
            handle_message(msg, tx).await;
        });

        future::ok(())
    });

    pin_mut!(write, messages);
    future::select(write, messages).await;

    warn!("disconnected from relay; will attempt to reconnect in 5 seconds");
    tokio::time::sleep(Duration::from_secs(5)).await;
}
