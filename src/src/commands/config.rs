use std::process::Command;

use anyhow::Result;
use repo::config::{get_config, get_config_path};

pub fn config() -> Result<()> {
    print!("{}", toml::to_string(&get_config()?)?);

    Ok(())
}

pub fn edit_config() -> Result<()> {
    // FIXME: use $EDITOR
    Command::new("hx").arg(get_config_path()?).status()?;

    Ok(())
}
