mod cli;
mod config;
mod github;
mod init;
mod pr;
mod query;

use crate::config::Config;
use crate::github::fetch_prs;
use crate::init::init;
use crate::pr::{build_pr_list, select_pr};
use crate::query::build_query;
use octocrab::Octocrab;
use std::process::exit;
use webbrowser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::cli().get_matches();
    if matches.subcommand_matches("init").is_some() {
        match init() {
            Ok(_) => {
                println!("Initialization completed successfully.");
                exit(0);
            }
            Err(e) => {
                eprintln!("Error during initialization: {}", e);
                eprintln!("Initialization failed. Please try again.");
                exit(1);
            }
        }
    }
    let config = Config::load().expect("Failed to load config");

    let username = matches.get_one::<String>("username").map(|s| s.as_str());
    let use_org = matches.get_flag("org") || config.organization_settings.always;
    let use_req = matches.get_flag("req");

    let query = build_query(username, &config, use_org, use_req);

    let octocrab = Octocrab::builder()
        .personal_token(config.token.clone())
        .build()?;

    let response = fetch_prs(&octocrab, &query).await?;

    match build_pr_list(&response) {
        Ok(pr_list) => match select_pr(pr_list) {
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
        },
        Err(e) => {
            eprintln!("Error building PR list: {}", e);
        }
    }

    Ok(())
}
