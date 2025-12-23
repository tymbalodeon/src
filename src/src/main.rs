mod commands;
mod config;

use clap::{Parser, Subcommand};
use commands::add::add;
use commands::config::config;
use commands::list::list;

/// Manage repositories in an organized way
#[derive(Parser)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Add local or clone remote repositories
    Add { repos: Vec<String> },

    /// Change directory to a repository
    Cd { repo: Option<String> },

    /// View config file
    Config,

    /// List repositories
    List,

    /// Create or initialize a new repository
    New { path: Option<String> },

    /// Remove repositories
    Remove { repo: Vec<String> },

    /// Sync repositories
    Sync { repo: Vec<String> },
}

fn main() {
    let result = match &Cli::parse().command {
        Some(Command::Add { repos }) => add(repos),

        Some(Command::Cd { repo: _ }) => {
            eprintln!("Implement cd!");

            Ok(())
        }

        Some(Command::Config) => config(),
        Some(Command::List) => {
            list();
            Ok(())
        },

        Some(Command::New { path: _ }) => {
            eprintln!("Implement new!");

            Ok(())
        }

        Some(Command::Remove { repo: _ }) => {
            eprintln!("Implement remove!");

            Ok(())
        }

        Some(Command::Sync { repo: _ }) => {
            eprintln!("Implement sync!");

            Ok(())
        }

        None => Ok(()),
    };

    match result {
        Ok(()) => (),
        Err(error) => eprintln!("{error}"),
    }
}
