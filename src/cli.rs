use clap::{Arg, Command};

pub fn cli() -> Command {
    Command::new("revq")
        .version("0.1.0")
        .arg(
            Arg::new("username")
                .short('u')
                .long("username")
                .value_name("USERNAME")
                .help("GitHub username")
                .required(false),
        )
        .arg(
            Arg::new("org")
                .short('o')
                .long("org")
                .help("Use organization PRs")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("req")
                .short('r')
                .long("req")
                .help("Show PRs where review is requested")
                .action(clap::ArgAction::SetTrue),
        )
}
