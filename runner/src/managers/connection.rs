use std::time::Duration;

use colored::Colorize;
use futures::{
    channel::mpsc::{self, UnboundedSender},
    pin_mut,
    StreamExt,
    TryStreamExt,
};
use log::{error, info, warn};
use spinners::{Spinner, Spinners};
use tokio_tungstenite::tungstenite::Message;

use crate::config_management::Configuration;

/// A manager to manage all network activity with the relay.
#[derive(Debug, Clone)]
pub(crate) struct ConnectionManager {
    config:     Configuration,
    write_sink: Option<UnboundedSender<Message>>,
}

impl ConnectionManager {
    pub(crate) fn new(config: Configuration) -> Self {
        Self {
            config,
            write_sink: None,
        }
    }

    pub(crate) fn send_message(&self, message: Message) {
        match &self.write_sink {
            Some(sink) => match sink.unbounded_send(message) {
                Ok(_) => {},
                Err(e) => {
                    error!("failed to send message: {}", e);
                },
            },
            None => {
                error!("no write sink available");
            },
        };
    }

    pub(crate) async fn connect_and_listen(&mut self) {
        loop {
            // attempt to connect to the relay
            let spinner = Spinner::new(Spinners::Dots, "Connecting to relay".to_string());
            let connection = tokio_tungstenite::connect_async(self.config.get_url()).await;
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
                    self.write_sink = Some(tx);
                    let messages = read.try_for_each(|msg| async {
                        let data = msg.into_data();
                        info!("received message: {:?}", data);
                        Ok(())
                    });

                    pin_mut!(write, messages);

                    futures::future::select(write, messages).await;

                    warn!("disconnected from relay; will attempt to reconnect in 5 seconds");
                    self.write_sink = None;
                    tokio::time::sleep(Duration::from_secs(5)).await;
                },
            }
        }
    }
}
