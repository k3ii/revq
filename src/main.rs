mod cli;
mod config;
mod github;
mod pr;
mod query;

use crate::config::Config;
use crate::github::fetch_prs;
use crate::pr::{build_pr_list, select_pr};
use crate::query::build_query;
use octocrab::Octocrab;
use webbrowser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::cli().get_matches();
    let config = Config::load().expect("Failed to load config");

    let username = matches.get_one::<String>("username").map(|s| s.as_str());
    let query = build_query(username, &config);

    let octocrab = Octocrab::builder()
        .personal_token(config.token.clone())
        .build()?;

    let response = fetch_prs(&octocrab, &query).await?;

    let pr_list = build_pr_list(&response);

    match select_pr(pr_list) {
        Some(selected_prs) if !selected_prs.is_empty() => {
            for pr in selected_prs {
                let url = pr.url.as_str();
                if !url.is_empty() {
                    if let Err(e) = webbrowser::open(url) {
                        eprintln!("Failed to open URL: {}", e);
                    }
                }
            }
        }
        Some(_) => {
            println!("No PR selected.");
        }
        None => {
            println!("Action aborted, no PR selected.");
        }
    }

    Ok(())
}
