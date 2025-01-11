use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub token: String,
    pub organizations: Vec<Organization>,
    #[serde(skip)]
    pub current_org: Option<String>,
    pub default_org: Option<String>,
    pub always_use_org: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub name: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        let config = std::fs::read_to_string(config_path)?;
        let mut config: Config = toml::from_str(&config)?;
        config.current_org = Self::load_current_org()?;
        Ok(config)
    }

    //    pub fn save(&self) -> Result<()> {
    //        let config_path = Self::config_path()?;
    //        let config_str = toml::to_string(self)?;
    //        std::fs::write(config_path, config_str)?;
    //        Ok(())
    //    }

    fn load_current_org() -> Result<Option<String>> {
        let path = Self::current_org_path()?;
        if path.exists() {
            let org = std::fs::read_to_string(path)?.trim().to_string();
            Ok(Some(org))
        } else {
            Ok(None)
        }
    }

    fn save_current_org(org: Option<&str>) -> Result<()> {
        let path = Self::current_org_path()?;
        if let Some(org) = org {
            std::fs::write(path, org)?;
        } else if path.exists() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }

    pub fn switch_org(&mut self, org_name: &str) -> Result<()> {
        match org_name {
            "default" => {
                self.current_org = self.default_org.clone();
            }
            "-" => {
                if let Some(current) = &self.current_org {
                    self.current_org = self
                        .organizations
                        .iter()
                        .find(|o| &o.name != current)
                        .map(|o| o.name.clone())
                        .or(self.default_org.clone());
                } else {
                    self.current_org = self.organizations.first().map(|o| o.name.clone());
                }
            }
            _ => {
                if !self.organizations.iter().any(|o| o.name == org_name) {
                    anyhow::bail!("Organization '{}' not found", org_name);
                }
                self.current_org = Some(org_name.to_string());
            }
        }

        Self::save_current_org(self.current_org.as_deref())?;
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let base_dirs = directories::BaseDirs::new().context("Failed to get base directories")?;
        Ok(base_dirs.config_dir().join("revq/config.toml"))
    }

    fn current_org_path() -> Result<PathBuf> {
        let base_dirs = directories::BaseDirs::new().context("Failed to get base directories")?;
        Ok(base_dirs.config_dir().join("revq/current"))
    }
}
