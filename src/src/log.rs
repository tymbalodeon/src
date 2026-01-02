use colored::Colorize;

pub enum LogLevel {
    Error,
    // Info,
    // Warning,
}

pub fn log(level: &LogLevel, message: &str) {
    let message = match level {
        LogLevel::Error => format!("{}: {message}", "error".bold().red()),
        // LogLevel::Warning => format!("warning: {message}"),
        // LogLevel::Info => format!("info: {message}"),
    };

    match level {
        LogLevel::Error
        // | LogLevel::Warning
        => eprintln!("{message}"),
        // LogLevel::Info => println!("{message}"),
    }
}
