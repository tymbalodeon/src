use std::path::PathBuf;

use dirs::{config_dir, home_dir};
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub root_directory: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root_directory: home_dir().unwrap().join("src"),
        }
    }
}

pub fn get_config() -> Config {
    Figment::from(Serialized::defaults(Config::default()))
        .merge(Toml::file(
            config_dir()
                .unwrap()
                .join("src/config.toml")
                .to_str()
                .unwrap(),
        ))
        .merge(Env::prefixed("SRC_"))
        .extract()
        .unwrap()
}
