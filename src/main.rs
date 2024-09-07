mod cli;
mod config;
mod github;
mod query;

use crate::config::Config;
use crate::github::fetch_prs;
use crate::query::build_query;
use octocrab::Octocrab;

#[tokio::main]
async fn main() {
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

    let response = fetch_prs(&octocrab, &query).await;

    match response {
        Ok(json) => {
            let formatted = serde_json::to_string_pretty(&json).unwrap();
            println!("response:\n{}", formatted);
        }
        Err(e) => {
            eprint!("Error fetching PRs: {:?}", e);
        }
    }
}
