use clap::Parser;
use human_panic::setup_panic;

use crate::managers::ConnectionManager;

/// A VLab relay runner. This app should run on your VLab instance, under your
/// account name. It is highly recommended that you run this in some sort of
/// detachable interface, such as zellij or screen.
#[derive(Parser, Debug)]
#[clap(name = "vlab relay runner", author, version, about, long_about = None, verbatim_doc_comment)]
struct Args {}

mod config_management;
mod handlers;
mod managers;
mod relay;
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
    let mut conn_manager = ConnectionManager::new(config);

    // connect to relay
    conn_manager.connect_and_listen().await;
}
