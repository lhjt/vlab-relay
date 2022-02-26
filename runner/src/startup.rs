use colored::Colorize;

pub(crate) fn print_header() {
    println!(
        "{} {}",
        "vlab-relay runner".green(),
        format!("v{}", env!("CARGO_PKG_VERSION")).blue()
    );
    println!(
        "{}\n",
        "Written by Jared (https://github.com/lhjt/vlab-relay)".truecolor(140, 140, 140)
    );
    println!(
        "{} {}",
        "This runner will run for".bright_blue().bold(),
        whoami::username().yellow().bold()
    );
}
