use crate::config::{Config, Organization};
use anyhow::{Context, Result};
use directories::BaseDirs;
use inquire::{Confirm, Text};
use std::fs;
use std::path::PathBuf;

pub fn init(force: bool) -> Result<InitResult> {
    let base_dirs = BaseDirs::new().context("Failed to get base directories")?;
    let config_dir = base_dirs.config_dir().join("revq");
    let config_file = config_dir.join("config.toml");

    if config_file.exists() && !force {
        return Ok(InitResult::Skipped(config_file));
    }

    let username = Text::new("Please enter your GitHub username:")
        .prompt()
        .context("Failed to get the username")?;

    let token = Text::new("Please enter your Github token:")
        .prompt()
        .context("Failed to get the token")?;

    let mut organizations = Vec::new();
    loop {
        let org_input =
            Text::new("Enter a Github organization name (or press Enter to finish):").prompt()?;

        if org_input.is_empty() {
            break;
        }

        organizations.push(Organization { name: org_input });
    }

    let always_use_org = if !organizations.is_empty() {
        Confirm::new("Do you want to always use organization context by default (without -o flag)?")
            .with_default(false)
            .prompt()?
    } else {
        false
    };

    let default_org = if !organizations.is_empty() {
        Text::new("Enter the name of the default organization (or press Enter for none):")
            .prompt()?
    } else {
        String::new()
    };

    let config = Config {
        username,
        token,
        organizations,
        current_org: None,
        default_org: if default_org.is_empty() {
            None
        } else {
            Some(default_org)
        },
        always_use_org,
    };

    let toml_output = toml::to_string(&config).context("Failed to serialize to TOML")?;

    fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
    fs::write(&config_file, toml_output).context("Failed to write config file")?;

    Ok(InitResult::Completed(config_file))
}

pub enum InitResult {
    Completed(PathBuf),
    Skipped(PathBuf),
}
