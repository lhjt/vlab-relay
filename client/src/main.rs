use clap::{Parser, Subcommand};

/// The VLab relay client. Allows you to execute commands on your VLab instance
/// from your local working environment. It automatically captures all of the
/// files in your cwd when you execute the command, and transfers this context
/// to your VLab instance.
#[derive(Parser, Debug)]
#[clap(name = "vlab-relay client", author, version, about, long_about = None, verbatim_doc_comment)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
    /// Optionally include this token on each command run. This is more useful
    /// for CI environments. Otherwise, prefer to use the `login` command to
    /// authenticate.
    token:   Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the 1511 style test suite on the specified file.
    Style { file: String },
    /// Run the autotest suite on the specified project.
    Test {
        course:     String,
        assignment: String,
    },
    /// Submit the specified file to the specified assignment.
    Give {
        /// The class the assignment belongs to.
        class:      String,
        /// The name of the assignment to submit to.
        assignment: String,
        /// The main entrypoint of the submission.
        entrypoint: String,
    },
    /// Save your token into your configuration file for use with requests.
    Login { token: String },
}

fn main() { Args::parse(); }
