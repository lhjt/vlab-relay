use std::time::Duration;

use colored::Colorize;
use futures::{channel::mpsc, future, pin_mut, StreamExt, TryStreamExt};
use log::warn;
use spinners::Spinner;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::handlers::message::handle_message;

pub(crate) async fn handle_connection(
    spinner: Spinner,
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
) {
    spinner.stop_with_message("✔ Connected to relay \n".green().to_string());

    // create proxy channel to relay messages
    let (tx, rx) = mpsc::unbounded();
    let (write, read) = stream.split();
    let write = rx.map(Ok).forward(write);

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
