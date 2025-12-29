use std::path::PathBuf;

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
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root_directory: home_dir().map(|home_dir| home_dir.join("src")),
        }
    }
}

pub fn get_config() -> Result<Config> {
    Figment::from(Serialized::defaults(Config::default()))
        .merge(Toml::file(
            config_dir()
                .context("failed to determine $XDG_CONFIG_HOME")?
                .join("src/config.toml")
                .to_str()
                .context("failed to decode the value of $XDG_CONFIG_DIR")?,
        ))
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
