use std::time::Duration;

use colored::Colorize;
use log::error;
use spinners::{Spinner, Spinners};

use crate::{config_management::Configuration, handlers::connection::handle_connection};

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
                Err(e) => handle_connection_error(spinner, e).await,
                Ok((stream, ..)) => handle_connection(spinner, stream).await,
            }
        }
    }
}

async fn handle_connection_error(spinner: Spinner, e: tokio_tungstenite::tungstenite::Error) {
    spinner.stop_with_message("❌ Failed to connect to relay\n".red().to_string());
    error!(
        "failed to connect to relay; will attempt to reconnect in 5 seconds: {}",
        e
    );
    tokio::time::sleep(Duration::from_secs(5)).await;
}
