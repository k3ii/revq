use clap::builder::styling;
use clap::{Arg, ArgGroup, Command};

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

pub fn cli() -> Command {
    Command::new("revq")
        .version("0.1.0")
        .styles(STYLES)
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
                .aliases(["review", "requested", "review-requested"])
                .help("Show PRs where review is requested")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("all")
                .long("all")
                .requires("org")
                .help("Show all PRs for organization (only works with --org)")
                .action(clap::ArgAction::SetTrue),
        )
        .group(
            ArgGroup::new("pr_filter")
                .args(["req", "all"])
                .multiple(false),
        )
        .subcommand(
            Command::new("init")
                .about("Initialize configuration file")
                .after_help("revq init should run only once."),
        )
}
