use clap::{Parser, Subcommand};

/// Manage git repositories in an organized way
#[derive(Parser)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Change directory
    Cd,

    /// Clone
    Clone,

    /// Config
    Config,

    /// List
    Ls,

    /// New
    New,

    /// Remove
    Rm,

    /// Sync
    Sync,
}

fn main() {
    match &Cli::parse().command {
        Some(Command::Cd {}) => {
            println!("Implement me!");
        }

        Some(Command::Clone {}) => {
            println!("Implement me!");
        }

        Some(Command::Config {}) => {
            println!("Implement me!");
        }

        Some(Command::Ls {}) => {
            println!("Implement me!");
        }

        Some(Command::New {}) => {
            println!("Implement me!");
        }

        Some(Command::Rm {}) => {
            println!("Implement me!");
        }

        Some(Command::Sync {}) => {
            println!("Implement me!");
        }

        None => {}
    }
}
