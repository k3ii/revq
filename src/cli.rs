use clap::{command, Arg, Command};

pub fn cli() -> Command {
    command!().arg(
        Arg::new("username")
            .short('u')
            .long("username")
            .value_name("USERNAME")
            .help("Specify a Github username")
            .value_parser(clap::value_parser!(String)),
    )
}
