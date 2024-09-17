use std::path::PathBuf;

use config::{Config, FileFormat};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::error::S2CliError;

#[derive(Debug, Deserialize, Serialize)]
pub struct S2Config {
    token: String,
}

/// Path to the configuration file
pub fn config_path() -> Result<PathBuf, S2CliError> {
    let mut path = dirs::config_dir().ok_or(S2ConfigError::ConfigDirNotFound)?;
    path.push("s2");
    path.push("config.toml");
    Ok(path)
}

#[allow(dead_code)]
pub fn load_config(path: &PathBuf) -> Result<S2Config, S2ConfigError> {
    let cfg = Config::builder()
        .add_source(config::File::new(
            path.to_str().ok_or(S2ConfigError::ConfigPathError)?,
            FileFormat::Toml,
        ))
        .build()
        .map_err(|_| S2ConfigError::ConfigLoadError)?;

    Ok(cfg
        .try_deserialize::<S2Config>()
        .map_err(|_| S2ConfigError::ConfigLoadError)?)
}

pub fn create_config(config_path: &PathBuf, token: &str) -> Result<(), S2ConfigError> {
    let cfg = S2Config {
        token: token.to_string(),
    };

    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).map_err(|_| S2ConfigError::ConfigWriteError)?;
    }

    let toml = toml::to_string(&cfg).unwrap();
    std::fs::write(config_path, toml).map_err(|_| S2ConfigError::ConfigWriteError)?;

    Ok(())
}

#[derive(Error, Debug)]
pub enum S2ConfigError {
    #[error("Failed to find config directory")]
    ConfigDirNotFound,
    #[error("Failed to find config file")]
    ConfigPathError,
    #[error("Failed to load config file")]
    ConfigLoadError,
    #[error("Failed to write config file")]
    ConfigWriteError,
}