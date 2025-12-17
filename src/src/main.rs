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
    /// Add local and remote repositories to the `src`-managed directory
    Add,

    /// Change directory to a managed repository
    Cd { repo: Option<String> },

    /// Clone a remote repositoryto the `src`-managed directory
    Clone { url: String },

    /// List managed repositories
    List,

    /// Create a new repository
    New,
}

fn main() {
    match &Cli::parse().command {
        Some(Command::Add) => {
            todo!()
        }

        Some(Command::Cd { repo: _ }) => {
            todo!()
        }

        Some(Command::Clone { url: _ }) => {
            todo!()
        }

        Some(Command::List) => {
            todo!()
        }

        Some(Command::New) => {
            todo!()
        }

        None => {}
    }
}
