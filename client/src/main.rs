#![warn(clippy::pedantic)]

use clap::{IntoApp, StructOpt};

use crate::cli::print_completions;

mod cli;
mod relay;

fn main() {
    let opts = cli::Args::parse();
    match opts.command {
        cli::Commands::Generate { generator } => {
            let mut cmd = cli::Args::command();
            print_completions(generator, &mut cmd);
        },
        _ => unimplemented!(),
    }
}
