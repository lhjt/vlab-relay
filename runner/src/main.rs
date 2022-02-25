use clap::Parser;
use colored::Colorize;
use futures::{pin_mut, StreamExt, TryStreamExt};
use human_panic::setup_panic;
use log::{error, info, warn};

/// A VLab relay runner. This app should run on your VLab instance, under your
/// account name. It is highly recommended that you run this in some sort of
/// detachable interface, such as zellij or screen.
#[derive(Parser, Debug)]
#[clap(name = "vlab relay runner", author, version, about, long_about = None, verbatim_doc_comment)]
struct Args {}

mod config_management;
mod startup;

#[tokio::main]
async fn main() {
    setup_panic!();
    simple_logger::init_with_level(log::Level::Info).expect("failed to initialize logger");
    Args::parse();

    // header output
    startup::print_header();

    // create config
    let config = config_management::get_config();

    // run relay
    // TODO: actual application logic

    loop {
        // attempt to connect to the relay
        let spinner =
            spinners::Spinner::new(spinners::Spinners::Dots, "Connecting to relay".to_string());
        let connection = tokio_tungstenite::connect_async(config.get_url()).await;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        match connection {
            // if we failed to connect, wait a bit and try again
            Err(e) => {
                spinner.stop_with_message("❌ Failed to connect to relay\n".red().to_string());
                error!(
                    "failed to connect to relay; will attempt to reconnect in 5 seconds: {}",
                    e
                );
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            },
            Ok((stream, ..)) => {
                spinner.stop_with_message("✔ Connected to relay \n".green().to_string());
                let (tx, rx) = futures::channel::mpsc::unbounded();
                let (write, read) = stream.split();

                let write = rx.map(Ok).forward(write);
                let messages = read.try_for_each(|msg| async {
                    let data = msg.into_data();
                    info!("received message: {:?}", data);
                    Ok(())
                });

                pin_mut!(write, messages);

                futures::future::select(write, messages).await;

                warn!("disconnected from relay; will attempt to reconnect in 5 seconds");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            },
        }
    }
}
