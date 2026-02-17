use std::process::Command;
use std::{env, path::PathBuf};

use anyhow::Result;
use repo::config::{get_config, get_config_path};

use crate::log::{log, LogLevel};

pub fn config(config_file: Option<&PathBuf>) -> Result<()> {
    print!("{}", toml::to_string(&get_config(config_file)?)?);

    Ok(())
}

pub fn edit_config() -> Result<()> {
    Command::new(env::var("EDITOR").unwrap_or_else(|_| "vi".to_string()))
        .arg(get_config_path()?)
        .status()?;

    Ok(())
}

pub fn get_config_value(
    config_file: Option<&PathBuf>,
    key: &str,
) -> Result<()> {
    let config = get_config(config_file)?;

    let value = match key {
        "host" => config.host.map_or(String::new(), |value| value),

        "owner" => config.owner.map_or(String::new(), |value| value),

        "root_directory" => config
            .root_directory
            .map_or(String::new(), |value| value.display().to_string()),

        _ => {
            log(&LogLevel::Error, &format!("key {key:?} does not exist"));

            String::new()
        }
    };

    print!("{value}");

    Ok(())
}
