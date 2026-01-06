use std::path::PathBuf;
use std::process::Command;

use dirs::{config_dir, home_dir};
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

use crate::error::SrcRepoError;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub host: Option<String>,
    pub owner: Option<String>,
    pub root_directory: Option<PathBuf>,
}

fn get_git_config_user(host: &str) -> Option<String> {
    if let Ok(result) = Command::new("git")
        .args(["config", &format!("{host}.user")])
        .output()
    {
        Some(
            String::from_utf8_lossy(&result.stdout)
                .to_string()
                .trim_end()
                .to_string(),
        )
    } else {
        None
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut username = get_git_config_user("github");

        username =
            username.map_or_else(|| get_git_config_user("gitlab"), Some);

        Self {
            root_directory: home_dir().map(|home_dir| home_dir.join("src")),
            host: Some("github.com".to_string()),
            owner: username,
        }
    }
}

/// # Errors
///
/// Will return `SrcRepoError` if it fails to parse the config file path
pub fn get_config_path() -> Result<String, SrcRepoError> {
    Ok(config_dir()
        .ok_or(SrcRepoError::Config)?
        .join("src/config.toml")
        .to_str()
        .ok_or(SrcRepoError::Config)?
        .to_string())
}

/// # Errors
///
/// Will return `SrcRepoError` if it fails to merge configuration from the file
/// and the environment
pub fn get_config() -> Result<Config, SrcRepoError> {
    Figment::from(Serialized::defaults(Config::default()))
        .merge(Toml::file(get_config_path()?))
        .merge(Env::prefixed("SRC_"))
        .extract()
        .map_or(Err(SrcRepoError::Config), Ok)
}

/// # Errors
///
/// Will return `SrcRepoError` if it fails to merge configuration from the file
/// and the environment
pub fn get_root_directory() -> Result<String, SrcRepoError> {
    Ok(get_config()?
        .root_directory
        .ok_or(SrcRepoError::Config)?
        .to_string_lossy()
        .to_string())
}

/// # Errors
///
/// Will return `SrcRepoError` if it fails to merge configuration from the file
/// and the environment
pub fn get_username() -> Result<String, SrcRepoError> {
    get_config()?.owner.ok_or(SrcRepoError::Config)
}
