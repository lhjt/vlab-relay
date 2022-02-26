use std::time::Duration;

use colored::Colorize;
use futures::{channel::mpsc, future, pin_mut, StreamExt, TryStreamExt};
use log::{error, info, warn};
use prost::bytes::Bytes;
use spinners::{Spinner, Spinners};
use tokio_tungstenite::tungstenite::Message;

use crate::{
    config_management::Configuration,
    managers::tasks::Task,
    relay::ws_extensions::{socket_frame::Data, SocketFrame},
};

/// A manager to manage all network activity with the relay.
#[derive(Debug, Clone)]
pub(crate) struct ConnectionManager {
    config: Configuration,
}

impl ConnectionManager {
    pub(crate) fn new(config: Configuration) -> Self { Self { config } }

    pub(crate) async fn connect_and_listen(&mut self) {
        loop {
            // attempt to connect to the relay
            let spinner = Spinner::new(Spinners::Dots, "Connecting to relay".to_string());
            let connection = tokio_tungstenite::connect_async(self.config.get_url().clone()).await;
            tokio::time::sleep(Duration::from_secs(1)).await;

            match connection {
                // if we failed to connect, wait a bit and try again
                Err(e) => {
                    spinner.stop_with_message("❌ Failed to connect to relay\n".red().to_string());
                    error!(
                        "failed to connect to relay; will attempt to reconnect in 5 seconds: {}",
                        e
                    );
                    tokio::time::sleep(Duration::from_secs(5)).await;
                },
                Ok((stream, ..)) => {
                    spinner.stop_with_message("✔ Connected to relay \n".green().to_string());
                    let (tx, rx) = mpsc::unbounded();
                    let (write, read) = stream.split();

                    let write = rx.map(Ok).forward(write);
                    let messages = read.try_for_each(|msg| {
                        let tx = tx.clone();

                        tokio::spawn(async move {
                            // determine the type of msg received
                            match msg {
                                Message::Binary(data) => {
                                    // attempt to parse the message as a task request
                                    match <SocketFrame as prost::Message>::decode(Bytes::from(data))
                                    {
                                        Ok(frame) => {
                                            // if we successfully parsed the message, then we can
                                            // process it
                                            match frame.data {
                                                Some(data) => {
                                                    if let Data::TaskRequest(req) = data {
                                                        info!("received task request: {}", req.id);

                                                        let response =
                                                            Task::from(req).execute().await;
                                                        info!(
                                                            "sending task response: {}",
                                                            response.id
                                                        );
                                                        let return_frame = SocketFrame {
                                                            data: Some(Data::TaskResponse(
                                                                response,
                                                            )),
                                                        };
                                                        tx.unbounded_send(Message::Binary(
                                                            prost::Message::encode_to_vec(
                                                                &return_frame,
                                                            ),
                                                        ))
                                                        .unwrap();
                                                    }
                                                },
                                                None => {
                                                    error!("received invalid message from relay");
                                                },
                                            }
                                        },
                                        Err(e) => {
                                            error!("failed to parse message: {}", e);
                                        },
                                    }
                                },
                                Message::Close(_) => {
                                    warn!("received close message from relay");
                                },
                                _ => {
                                    // should not be receiving any other message types
                                    warn!("received unexpected message type");
                                },
                            }
                        });

                        future::ok(())
                    });

                    pin_mut!(write, messages);

                    futures::future::select(write, messages).await;

                    warn!("disconnected from relay; will attempt to reconnect in 5 seconds");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                },
            }
        }
    }
}
