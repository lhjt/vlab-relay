use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password};
use human_panic::setup_panic;

/// A VLab relay runner. This app should run on your VLab instance, under your
/// account name. It is highly recommended that you run this in some sort of
/// detachable interface, such as zellij or screen.
#[derive(Parser, Debug)]
#[clap(name = "vlab relay runner", author, version, about, long_about = None, verbatim_doc_comment)]
struct Args {}

fn main() {
    setup_panic!();
    Args::parse();

    let host = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Please enter a hostname")
        .default("vlab-relay.example.com".into())
        .interact_text()
        .unwrap();
    let secure = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Use secure connection?")
        .default(true)
        .interact()
        .unwrap();
    let token = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Please enter your token")
        .interact()
        .unwrap();
}
