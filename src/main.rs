mod cli;
mod config;
mod github;
mod query;

use crate::config::Config;
use crate::github::fetch_prs;
use crate::query::build_query;
use octocrab::Octocrab;

fn main() {
    let matches = cli::cli().get_matches();
    let config = Config::load().expect("Failed to load config");

    let username = matches.get_one::<String>("username");
    let query = build_query(username.map(|x| x.as_str()), &config);

    let octocrab = match Octocrab::builder()
        .personal_token(config.token.clone())
        .build()
    {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to build Octocrab: {}", e);
            return;
        }
    };

}
