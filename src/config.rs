use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub base_worktree_path: PathBuf,
    pub naming: Naming,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
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
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = match dirs::home_dir() {
            Some(home) => home.join(".config").join("git-wt").join("config.json"),
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
}
