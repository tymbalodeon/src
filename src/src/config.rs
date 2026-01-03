use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result};
use dirs::{config_dir, home_dir};
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub root_directory: Option<PathBuf>,
    pub username: Option<String>,
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
            username,
        }
    }
}

fn get_config_path() -> Result<String> {
    Ok(config_dir()
        .context("failed to determine $XDG_CONFIG_HOME")?
        .join("src/config.toml")
        .to_str()
        .context("failed to decode the value of $XDG_CONFIG_DIR")?
        .to_string())
}

pub fn get_config() -> Result<Config> {
    Figment::from(Serialized::defaults(Config::default()))
        .merge(Toml::file(get_config_path()?))
        .merge(Env::prefixed("SRC_"))
        .extract()
        .context("failed to generate configuration")
}

pub fn get_root_directory() -> Result<String> {
    Ok(get_config()?
        .root_directory
        .context("failed to determine root directory")?
        .to_string_lossy()
        .to_string())
}

pub fn edit_config() -> Result<()> {
    Command::new("hx").arg(get_config_path()?).status()?;

    Ok(())
}
