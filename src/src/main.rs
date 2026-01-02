mod commands;
mod config;
mod log;

use clap::{Parser, Subcommand};
use commands::add::add;
use commands::config::config;
use commands::list::list;

use crate::commands::list::{
    hosts, list_all, list_non_managed, names, owners, SortByOption,
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
    /// List managed and non-managed repositories
    All {
        /// Include hidden directories when searching for non-managed repositories
        #[arg(long)]
        include_hidden: bool,

        /// Filter to repositories with host partially matching this value
        #[arg(long)]
        host: Option<String>,

        /// Filter to repositories with owner partially matching this value
        #[arg(long)]
        owner: Option<String>,

        /// Filter to repositories with name partially matching this value
        #[arg(long)]
        name: Option<String>,

        /// (Not compatible with `--path`) Don't display host values
        #[arg(long)]
        no_host: bool,

        /// (Not compatible with `--path`) Don't display owner values
        #[arg(long)]
        no_owner: bool,

        /// List as paths
        #[arg(long)]
        path: bool,

        #[arg(long)]
        sort_by: Option<SortByOption>,
    },

    /// Show all hosts
    Hosts,

    /// Show all repository names
    Names,

    /// List repositories in $HOME that are not managed by `src`
    NonManaged {
        /// Include hidden directories when searching for non-managed repositories
        #[arg(long)]
        include_hidden: bool,

        /// Filter to repositories with host partially matching this value
        #[arg(long)]
        host: Option<String>,

        /// Filter to repositories with owner partially matching this value
        #[arg(long)]
        owner: Option<String>,

        /// Filter to repositories with name partially matching this value
        #[arg(long)]
        name: Option<String>,

        /// (Not compatible with `--path`) Don't display host values
        #[arg(long)]
        no_host: bool,

        /// (Not compatible with `--path`) Don't display owner values
        #[arg(long)]
        no_owner: bool,

        /// List as paths
        #[arg(long)]
        path: bool,

        #[arg(long)]
        sort_by: Option<SortByOption>,
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

        /// Filter to repositories with host partially matching this value
        #[arg(long)]
        host: Option<String>,

        /// Filter to repositories with owner partially matching this value
        #[arg(long)]
        owner: Option<String>,

        /// Filter to repositories with name partially matching this value
        #[arg(long)]
        name: Option<String>,

        /// (Not compatible with `--path`) Don't display host values
        #[arg(long)]
        no_host: bool,

        /// (Not compatible with `--path`) Don't display owner values
        #[arg(long)]
        no_owner: bool,

        /// List as paths
        #[arg(long)]
        path: bool,

        #[arg(long)]
        sort_by: Option<SortByOption>,
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

            Some(ListSubcommand::All {
                include_hidden,
                host,
                owner,
                name,
                no_host,
                no_owner,
                path,
                sort_by,
            }) => list_all(
                *include_hidden,
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
                sort_by,
            }) => list_non_managed(
                *include_hidden,
                host.as_ref(),
                owner.as_ref(),
                name.as_ref(),
                *no_host,
                *no_owner,
                *path,
                sort_by.as_ref(),
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
