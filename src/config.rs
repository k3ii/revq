use anyhow::{Context, Result};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub token: String,
    pub organization: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let base_dir = BaseDirs::new().context("Failed to get base directories")?;
        let config_path = base_dir.config_dir().join("revq/revq.toml");
        let config = std::fs::read_to_string(config_path.clone())
            .with_context(|| format!("Failed to read config file at {:?}", config_path))?;
        let config: Config = toml::from_str(&config).context("Failed to parse config file")?;
        Ok(config)
    }
}
