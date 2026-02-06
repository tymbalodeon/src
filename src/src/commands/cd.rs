use anyhow::Result;

use crate::log::{log, LogLevel};

pub fn cd() -> Result<()> {
    log(&LogLevel::Error, "The shell hook has not been initialized.");

    Ok(())
}
