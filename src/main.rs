mod cli;
mod config;
mod query;

use query::build_query;

use crate::config::Config;

fn main() {
    let matches = cli::cli().get_matches();
    let config = Config::load().expect("Failed to load config");

    let username = matches.get_one::<String>("username");
    let _query = build_query(username.map(|x| x.as_str()), &config);
}
