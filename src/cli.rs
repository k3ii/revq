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
        .author("Jain Ramchurn")
        .about("Review and query your pull requests")
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
                .arg(
                    Arg::new("force")
                    .long("force")
                    .help("Overwrite existing configuration file")
                    .action(clap::ArgAction::SetTrue)
                )
                .after_help(
                    "The 'init' subcommand creates a new configuration file for revq. \
                    It will prompt you for your GitHub username, personal access token, \
                    and optional organization settings.\n\n\
                    Usage:\n\
                    - First-time setup: 'revq init'\n\
                    - Recreate config:   'revq init --force'\n\n\
                    Note: Running 'revq init' without --force will not overwrite an existing config. \
                    Use --force with caution as it will replace your current configuration."
                ),
        )
        .subcommand(
            Command::new("switch")
                .about("Switch between organizations")
                .arg(
                    Arg::new("org")
                        .help("Organization name to switch to, 'default' for default org, or '-' to toggle")
                        .required(true)
                        .index(1)
                )
        )
}
