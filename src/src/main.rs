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

    /// Change directory to a managed repository
    Cd { repo: Option<String> },

    /// Clone a remote repository
    Clone { url: String },

    /// Initialize a new repository in the current directory
    Init,

    /// List managed repositories
    List,

    /// Create a new repository
    New { path: String },

    /// Remove a repository
    Remove { repo: String },

    /// Sync a repository
    Sync { repo: Option<String> },
}

fn main() {
    match &Cli::parse().command {
        Some(Command::Add { repo: _ }) => {
            todo!()
        }

        Some(Command::Cd { repo: _ }) => {
            todo!()
        }

        Some(Command::Clone { url: _ }) => {
            todo!()
        }
        Some(Command::Init) => {
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
