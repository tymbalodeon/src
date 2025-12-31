mod commands;
mod config;
mod log;

use clap::{Parser, Subcommand};
use commands::add::add;
use commands::config::config;
use commands::list::list;

use crate::commands::list::{
    hosts, list_non_managed, names, owners, SortByComponent,
};

/// Manage repositories in an organized way
#[derive(Parser)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum ListSubcommand {
    /// Show all hosts
    Hosts,

    /// Show all repository names
    Names,

    NonManaged {
        #[arg(long)]
        include_hidden: bool,

        #[arg(long)]
        host: Option<String>,

        #[arg(long)]
        owner: Option<String>,

        #[arg(long)]
        name: Option<String>,

        #[arg(long)]
        no_host: bool,

        #[arg(long)]
        no_owner: bool,

        #[arg(long)]
        path: bool,
    },

    /// Show all owners
    Owners,
}

#[derive(Subcommand)]
enum Command {
    /// Add local or clone remote repositories
    Add {
        repos: Vec<String>,

        // TODO
        // #[arg(long)]
        // cd: bool,
        #[arg(long)]
        force: bool,
        // TODO
        // #[arg(long)]
        // open: bool,
    },

    /// Change directory to a repository
    Cd { repo: Option<String> },

    /// View config file
    Config,

    /// List repositories
    List {
        #[command(subcommand)]
        command: Option<ListSubcommand>,

        #[arg(long)]
        host: Option<String>,

        #[arg(long)]
        owner: Option<String>,

        #[arg(long)]
        name: Option<String>,

        #[arg(long)]
        no_host: bool,

        #[arg(long)]
        no_owner: bool,

        #[arg(long)]
        path: bool,

        #[arg(long)]
        sort_by: Option<SortByComponent>,
    },

    /// Create or initialize a new repository
    New { path: Option<String> },

    /// Remove repositories
    Remove { repo: Vec<String> },

    /// Sync repositories
    Sync { repo: Vec<String> },
}

fn main() {
    let result = match &Cli::parse().command {
        Some(Command::Add { repos, force }) => add(repos, *force),

        Some(Command::Cd { repo: _ }) => {
            eprintln!("Implement cd!");

            Ok(())
        }

        Some(Command::Config) => config(),

        Some(Command::List {
            command,
            host,
            owner,
            name,
            no_host,
            no_owner,
            path,
            sort_by,
        }) => match command {
            None => list(
                host.as_ref(),
                owner.as_ref(),
                name.as_ref(),
                *no_host,
                *no_owner,
                *path,
                sort_by.as_ref(),
            ),

            Some(ListSubcommand::Hosts) => hosts(),
            Some(ListSubcommand::Names) => names(),

            Some(ListSubcommand::NonManaged {
                include_hidden,
                host,
                owner,
                name,
                no_host,
                no_owner,
                path,
            }) => list_non_managed(
                *include_hidden,
                host.as_ref(),
                owner.as_ref(),
                name.as_ref(),
                *no_host,
                *no_owner,
                *path,
            ),

            Some(ListSubcommand::Owners) => owners(),
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
