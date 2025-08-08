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
        Some(Command::Cd) => {
            todo!()
        }

        Some(Command::Clone) => {
            todo!()
        }

        Some(Command::Config) => {
            todo!()
        }

        Some(Command::Ls) => {
            todo!()
        }

        Some(Command::New) => {
            todo!()
        }

        Some(Command::Rm) => {
            todo!()
        }

        Some(Command::Sync) => {
            todo!()
        }

        None => {}
    }
}
