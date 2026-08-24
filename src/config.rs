use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub base_worktree_path: PathBuf,
    pub naming: Naming,
    pub default_branch: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Naming {
    #[default]
    Structured,
    Flat,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_worktree_path: dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("worktrees"),
            naming: Naming::default(),
            default_branch: None,
        }
    }
}

impl Config {
    pub fn config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".config").join("git-wt").join("config.json"))
    }

    pub fn load() -> Self {
        let config_path = match Self::config_path() {
            Some(path) => path,
            None => return Self::default(),
        };

        if !config_path.exists() {
            return Self::default();
        }

        match Self::load_from(&config_path) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Warning: Failed to parse git-wt config: {e}");
                Self::default()
            }
        }
    }

    pub fn init(force: bool) -> Result<PathBuf> {
        let path = Self::config_path().context("Could not determine user home directory")?;
        if path.exists() && !force {
            anyhow::bail!(
                "Config file already exists at {}. Use --force to overwrite.",
                path.display()
            );
        }

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory {}", parent.display())
            })?;
        }

        let default_wt = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("worktrees");

        let template_config = Config {
            base_worktree_path: default_wt,
            naming: Naming::Structured,
            default_branch: None,
        };

        let json =
            serde_json::to_string_pretty(&template_config).context("Failed to serialize config")?;
        std::fs::write(&path, json)
            .with_context(|| format!("Failed to write config file {}", path.display()))?;

        Ok(path)
    }

    pub fn reset() -> Result<PathBuf> {
        let path = Self::config_path().context("Could not determine user home directory")?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory {}", parent.display())
            })?;
        }

        let default_config = Config::default();
        let json = serde_json::to_string_pretty(&default_config)
            .context("Failed to serialize config")?;
        std::fs::write(&path, json)
            .with_context(|| format!("Failed to write config file {}", path.display()))?;

        Ok(path)
    }

    fn load_from(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config from {}", path.display()))?;

        let config: Config = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse config from {}", path.display()))?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let config = Config::default();
        assert!(config.base_worktree_path.to_string_lossy().contains("worktrees"));
        assert_eq!(config.naming, Naming::Structured);
        assert_eq!(config.default_branch, None);
    }

    #[test]
    fn naming_deserialize() {
        let json = r#"{"naming": "flat"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.naming, Naming::Flat);
    }

    #[test]
    fn naming_default_on_missing() {
        let json = r#"{"base_worktree_path": "/tmp/wt"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.naming, Naming::Structured);
    }

    #[test]
    fn default_branch_deserialize() {
        let json = r#"{"default_branch": "develop"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.default_branch, Some("develop".to_string()));
    }

    #[test]
    fn default_branch_null() {
        let json = r#"{"default_branch": null}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.default_branch, None);
    }
}
