use anyhow::{Context, Result};
use repo::list::list_repos;

use crate::config::get_config;

pub fn list(
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    no_host: bool,
    no_owner: bool,
    path: bool,
) -> Result<()> {
    let root_directory = get_config()?
        .root_directory
        .context("failed to determine root directory")?;

    print!(
        "{}",
        list_repos(
            root_directory.to_string_lossy().as_ref(),
            host,
            owner,
            name,
            no_host,
            no_owner,
            path
        )?
        .join("\n")
    );

    Ok(())
}
