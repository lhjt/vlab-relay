use std::io;

use clap::{ArgEnum, Command, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};

#[allow(clippy::doc_markdown)]
/// The VLab relay client. Allows you to execute commands on your VLab
/// instance from your local working environment. It automatically
/// captures all of the files in your cwd when you execute the command,
/// and transfers this context to your VLab instance.
#[derive(Parser, Debug)]
#[clap(
    name = "vlab-relay client",
    author,
    version,
    about,
    long_about = None,
    verbatim_doc_comment,
    trailing_var_arg = true
)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub(crate) command: Commands,
    /// Optionally include this token on each command run. This is more
    /// useful for CI environments. Otherwise, prefer to use the
    /// `login` command to authenticate.
    pub(crate) token:   Option<String>,
}

pub(crate) fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// If provided, outputs the completion file for given shell
    Generate {
        #[clap(arg_enum)]
        generator: Shell,
    },
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
    Config {
        #[clap(arg_enum)]
        item:  Item,
        value: String,
    },
}

#[derive(Copy, Clone, ArgEnum, Debug)]
/// Save or get the value from your configuration file.
pub(crate) enum Item {
    Token,
    Uri,
}
