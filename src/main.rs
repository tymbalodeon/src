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

    /// Clone a remote repositoryto the `src`-managed directory
    Clone { url: String },
}

fn main() {
    match &Cli::parse().command {
        Some(Command::Add) => {
            todo!()
        }

        Some(Command::Clone { url: _ }) => {
            todo!()
        }

        None => {}
    }
}
