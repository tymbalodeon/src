mod commands;
mod log;
mod repo;

use clap::{Parser, Subcommand};
use commands::add::add;
use commands::config::config;
use commands::list::list;

use crate::commands::{
    config::{edit_config, get_config_value},
    hook::hook,
    list::{hosts, list_all, list_non_managed, names, owners, SortByOption},
    remove::remove,
};

/// Manage repositories in an organized way
#[derive(Parser)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum ConfigSubcommand {
    /// Open config file in $EDITOR
    Edit,

    /// Get a config value
    Get { key: String },
}

#[derive(Subcommand)]
enum ListSubcommand {
    /// List managed and unmanaged repositories
    All {
        /// Include hidden directories when searching for unmanaged repositories
        #[arg(long)]
        hidden: bool,
    },

    /// Show all hosts
    Hosts {
        /// Show hosts for unmanaged as well as managed repositories
        #[arg(long)]
        all: bool,

        /// Include hidden directories when searching for unmanaged repositories
        #[arg(long)]
        hidden: bool,
    },

    /// Show all repository names
    Names {
        /// Show hosts for unmanaged as well as managed repositories
        #[arg(long)]
        all: bool,

        /// Include hidden directories when searching for unmanaged repositories
        #[arg(long)]
        hidden: bool,
    },

    /// List repositories in $HOME that are not managed by `src`
    NonManaged {
        /// Include hidden directories when searching for unmanaged repositories
        #[arg(long)]
        hidden: bool,
    },

    /// Show all owners
    Owners {
        /// Show hosts for unmanaged as well as managed repositories
        #[arg(long)]
        all: bool,

        /// Include hidden directories when searching for unmanaged repositories
        #[arg(long)]
        hidden: bool,
    },
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
    Config {
        #[command(subcommand)]
        command: Option<ConfigSubcommand>,
    },

    /// Generate shell hook (required for `cd`)
    Hook,

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

        /// Filter to repositories with owner matching the value of config.username
        #[arg(long)]
        me: bool,

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
    Remove { repos: Vec<String> },

    /// Sync repositories
    Sync { repos: Vec<String> },
}

fn main() {
    let result = match &Cli::parse().command {
        Some(Command::Add { repos, force }) => add(repos, *force),

        Some(Command::Cd { repo: _ }) => {
            eprintln!("Implement cd!");

            Ok(())
        }

        Some(Command::Config { command }) => {
            command
                .as_ref()
                .map_or_else(config, |command| match command {
                    ConfigSubcommand::Edit => edit_config(),
                    ConfigSubcommand::Get { key } => get_config_value(key),
                })
        }

        Some(Command::Hook) => hook(),

        Some(Command::List {
            command,
            host,
            owner,
            me,
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
                *me,
                *no_host,
                *no_owner,
                *path,
                sort_by.as_ref(),
            ),

            Some(ListSubcommand::All { hidden }) => list_all(
                *hidden,
                host.as_ref(),
                owner.as_ref(),
                name.as_ref(),
                *me,
                *no_host,
                *no_owner,
                *path,
                sort_by.as_ref(),
            ),

            Some(ListSubcommand::Hosts { all, hidden }) => {
                hosts(*all, *hidden)
            }

            Some(ListSubcommand::Names { all, hidden }) => {
                names(*all, *hidden, *me)
            }

            Some(ListSubcommand::NonManaged { hidden }) => list_non_managed(
                *hidden,
                host.as_ref(),
                owner.as_ref(),
                name.as_ref(),
                *me,
                *no_host,
                *no_owner,
                *path,
                sort_by.as_ref(),
            ),

            Some(ListSubcommand::Owners { all, hidden }) => {
                owners(*all, *hidden)
            }
        },

        Some(Command::New { path: _ }) => {
            eprintln!("Implement new!");

            Ok(())
        }

        Some(Command::Remove { repos }) => remove(repos),

        Some(Command::Sync { repos: _ }) => {
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
