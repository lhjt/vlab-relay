use clap::Parser;

/// A VLab relay runner. This app should run on your VLab instance, under your
/// account name. It is highly recommended that you run this in some sort of
/// detachable interface, such as zellij or screen.
#[derive(Parser, Debug)]
#[clap(name = "vlab relay runner", author, version, about, long_about = None, verbatim_doc_comment)]
struct Args {}

fn main() { Args::parse(); }
