use anyhow::Result;

use crate::config::get_config;

pub fn config() -> Result<()> {
    print!("{}", toml::to_string(&get_config()?)?);

    Ok(())
}
