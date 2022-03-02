#![warn(clippy::pedantic)]

use clap::{IntoApp, StructOpt};

use crate::cli::print_completions;

mod cli;
mod config;
mod relay;

fn main() {
    let opts = cli::Args::parse();
    if let cli::Commands::Generate { generator } = opts.command {
        let mut cmd = cli::Args::command();
        print_completions(generator, &mut cmd);
    } else {
        let config = config::get_config();
        config.save().unwrap();
    }
}
