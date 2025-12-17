use clap::{Parser, Subcommand};

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
    Add { repo: String },

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
    match &Cli::parse().command {
        Some(Command::Add { repo: _ }) => {
            todo!()
        }

        Some(Command::Cd { repo: _ }) => {
            todo!()
        }

        Some(Command::Config) => {
            todo!()
        }

        Some(Command::List) => {
            todo!()
        }

        Some(Command::New { path: _ }) => {
            todo!()
        }

        Some(Command::Remove { repo: _ }) => {
            todo!()
        }

        Some(Command::Sync { repo: _ }) => {
            todo!()
        }

        None => {}
    }
}
