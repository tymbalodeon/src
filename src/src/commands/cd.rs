use crate::log::{LogLevel, log};

pub fn cd() {
    log(&LogLevel::Error, "The shell hook has not been initialized.");
}
