use clap::Parser;
use human_panic::setup_panic;

/// A VLab relay runner. This app should run on your VLab instance, under your
/// account name. It is highly recommended that you run this in some sort of
/// detachable interface, such as zellij or screen.
#[derive(Parser, Debug)]
#[clap(name = "vlab relay runner", author, version, about, long_about = None, verbatim_doc_comment)]
struct Args {}

mod config_management;
mod startup;

fn main() {
    setup_panic!();
    Args::parse();

    // header output
    startup::print_header();

    // create config
    let config = config_management::get_config();

    // run relay
    // TODO: actual application logic
}
