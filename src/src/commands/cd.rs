use crate::log::{log, LogLevel};

pub fn cd() {
    log(&LogLevel::Error, "The shell hook has not been initialized.");
}
