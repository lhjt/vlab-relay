#![warn(clippy::pedantic)]

use clap::StructOpt;

mod cli;
mod relay;

fn main() { cli::Args::parse(); }
