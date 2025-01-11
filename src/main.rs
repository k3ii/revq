mod cli;
mod config;
mod github;
mod init;
mod pr;
mod query;

use crate::config::Config;
use crate::github::fetch_prs;
use crate::init::{init, InitResult};
use crate::pr::{build_pr_list, select_pr};
use crate::query::build_query;
use octocrab::Octocrab;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::cli().get_matches();

    if let Some(init_matches) = matches.subcommand_matches("init") {
        let force = init_matches.get_flag("force");
        match init(force) {
            Ok(InitResult::Completed(path)) => {
                println!("Initialization completed successfully.");
                println!("Configuration saved to: {}", path.display());
                return Ok(());
            }
            Ok(InitResult::Skipped(path)) => {
                println!("Configuration file already exists at: {}", path.display());
                println!("Use 'revq init --force' to overwrite the existing configuration.");
                return Ok(());
            }
            Err(e) => {
                eprintln!("Error during initialization: {}", e);
                eprintln!("Initialization failed. Please try again.");
                std::process::exit(1);
            }
        }
    } else if let Some(switch_matches) = matches.subcommand_matches("switch") {
        let org = switch_matches.get_one::<String>("org").unwrap();
        let mut config = Config::load()?;
        config.switch_org(org)?;
        println!(
            "Switched to organization: {}",
            config.current_org.as_deref().unwrap_or("default")
        );
        return Ok(());
    }

    let config = Config::load().expect("Failed to load config");

    let username = matches.get_one::<String>("username").map(|s| s.as_str());
    //let use_org = matches.get_flag("org") || config.organizations.iter().any(|o| o.always);
    let use_org = matches.get_flag("org")
        || config.default_org.as_ref().map_or(false, |default_org| {
            config.organizations.iter().any(|o| o.name == *default_org)
        });
    let use_req = matches.get_flag("req");
    let show_all = matches.get_flag("all");

    let query = build_query(username, &config, use_org, use_req, show_all);

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
