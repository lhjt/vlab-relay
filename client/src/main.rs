#![warn(clippy::pedantic)]

use clap::{IntoApp, StructOpt};

use crate::cli::print_completions;

mod cli;
mod config;
mod relay;

fn main() {
    let opts = cli::Args::parse();
    let mut config = config::get_config();

    match opts.command {
        cli::Commands::Generate { generator } => {
            let mut cmd = cli::Args::command();
            print_completions(generator, &mut cmd);
        },
        cli::Commands::Config { item, value } => {
            match item {
                cli::Item::Token => {
                    config.token = Some(value);
                },
                cli::Item::Uri => {
                    config.uri = Some(value);
                },
            }

            config.save().unwrap();
        },
        _ => {
            // Check if any of the config values are missing.
            config.check_complete();
        },
    }
}
