use anyhow::{Context, Result};
use colored::*;
use directories::BaseDirs;
use inquire::{Confirm, Text};
use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use toml;

#[derive(Serialize)]
struct OrgSet {
    org: Option<String>,
    always: bool,
}

#[derive(Serialize)]
struct UserInfo {
    username: String,
    token: String,
    organization_settings: Option<OrgSet>,
}

fn show_spinner() {
    let spinner_chars = ["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];
    let spinner_interval = Duration::from_millis(50);

    for _ in 0..3 {
        for &char in &spinner_chars {
            print!("\r{}", char.blue().bold());
            std::io::stdout().flush().expect("Failed to flush stdout");
            thread::sleep(spinner_interval);
        }
    }
}

fn save_to_xdg_config(content: &str) -> Result<PathBuf> {
    let spinner_handle = thread::spawn(show_spinner);

    let base_dirs = BaseDirs::new().context("Failed to get base directories")?;
    let config_dir = base_dirs.config_dir().join("revq");

    fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
    let config_file = config_dir.join("config.toml");
    fs::write(&config_file, content).context("Failed to write config file")?;

    spinner_handle.join().expect("Spinner thread panicked");

    Ok(config_file)
}

fn prompt_for_user_info() -> Result<UserInfo> {
    let username = Text::new("Please enter your GitHub username:")
        .prompt()
        .context("Failed to get the username")?;

    let token = Text::new("Please enter your Github token:")
        .prompt()
        .context("Failed to get the token")?;

    let org_input = Text::new("Enter your Github organization name (optional):")
        .prompt_skippable()
        .context("Failed to get the organization")?;

    let org = org_input.filter(|s| !s.is_empty());

    let organization_settings = if org.is_some() {
        let always = Confirm::new("Do you want to always use the organization context by default?")
            .with_default(false)
            .prompt()
            .context("Error while prompting for always use org setting")?;

        Some(OrgSet { org, always })
    } else {
        None
    };

    Ok(UserInfo {
        username,
        token,
        organization_settings,
    })
}

pub fn init() -> Result<()> {
    let user_info = prompt_for_user_info()?;

    let toml_output = toml::to_string(&user_info).context("Failed to serialize to TOML")?;

    match save_to_xdg_config(&toml_output) {
        Ok(path) => println!("\rConfiguration saved to: {}", path.display()),
        Err(e) => eprintln!("Failed to save configuration: {}", e),
    }

    Ok(())
}
