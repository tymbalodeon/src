mod commands;
mod log;
mod repo;

use clap::{Parser, Subcommand};
use commands::{
    add::add,
    cd::cd,
    config::{config, edit_config, get_config_value},
    hook::hook,
    list::list,
    list::{SortByOption, hosts, list_all, list_unmanaged, names, owners},
    remove::remove,
};

/// Manage source code repositories
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
    Unmanaged {
        /// Include hidden directories when searching for unmanaged repositories
        #[arg(long)]
        hidden: bool,

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

        /// Filter to repositories with host partially matching this value
        #[arg(long)]
        host: Option<String>,

        /// Filter to repositories with owner partially matching this value
        #[arg(long)]
        owner: Option<String>,

        /// Filter to repositories with owner matching the value of config.username
        #[arg(long)]
        me: bool,

        // TODO
        // #[arg(long)]
        // cd: bool,
        #[arg(long)]
        force: bool,
        // TODO
        // #[arg(long)]
        // open: bool,
    },

    /// Open the remote repository web page in the browser
    Browse,

    /// Change directory to a repository (requires shell hook -- see `hook`)
    Cd {
        /// Filter to repositories with host partially matching this value
        #[arg(long)]
        host: Option<String>,

        /// Filter to repositories with name partially matching this value
        #[arg(long)]
        name: Option<String>,

        /// Filter to repositories with owner partially matching this value
        #[arg(long)]
        owner: Option<String>,

        /// Repository name
        repo: Option<String>,
    },

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
    Remove {
        repos: Vec<String>,

        /// Filter to repositories with host partially matching this value
        #[arg(long)]
        host: Option<String>,

        /// Filter to repositories with owner partially matching this value
        #[arg(long)]
        owner: Option<String>,

        /// Filter to repositories with owner matching the value of config.username
        #[arg(long)]
        me: bool,

        #[arg(long)]
        force: bool,
    },

    /// Sync repositories
    Sync { repos: Vec<String> },
}

fn main() {
    let result = match &Cli::parse().command {
        Some(Command::Add {
            repos,
            host,
            owner,
            me,
            force,
        }) => add(repos, host.as_ref(), owner.as_ref(), *me, *force),

        Some(Command::Browse) => {
            println!("Implement me!");

            Ok(())
        }

        Some(Command::Cd {
            repo: _,
            host: _,
            name: _,
            owner: _,
        }) => {
            cd();

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

        Some(Command::Hook) => {
            hook();

            Ok(())
        }

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

            Some(ListSubcommand::All {
                hidden,
                host: all_host,
                owner: all_owner,
                me: all_me,
                name: all_name,
                no_host: all_no_host,
                no_owner: all_no_owner,
                path: all_path,
                sort_by: all_sort_by,
            }) => {
                let host = all_host.as_ref().map_or(host, |_| all_host);
                let me = *all_me || *me;
                let name = all_name.clone().map_or(name, |_| all_name);
                let no_host = *all_no_host || *no_host;
                let no_owner = *all_no_owner || *no_owner;
                let owner = all_owner.clone().map_or(owner, |_| all_owner);
                let path = *all_path || *path;
                let sort_by =
                    all_sort_by.clone().map_or(sort_by, |_| all_sort_by);

                list_all(
                    *hidden,
                    host.as_ref(),
                    owner.as_ref(),
                    name.as_ref(),
                    me,
                    no_host,
                    no_owner,
                    path,
                    sort_by.as_ref(),
                )
            }

            Some(ListSubcommand::Hosts { all, hidden }) => {
                hosts(*all, *hidden)
            }

            Some(ListSubcommand::Names { all, hidden }) => {
                names(*all, *hidden, *me)
            }

            Some(ListSubcommand::Unmanaged {
                hidden,
                host: all_host,
                owner: all_owner,
                me: all_me,
                name: all_name,
                no_host: all_no_host,
                no_owner: all_no_owner,
                path: all_path,
                sort_by: all_sort_by,
            }) => {
                let host = all_host.as_ref().map_or(host, |_| all_host);
                let me = *all_me || *me;
                let name = all_name.clone().map_or(name, |_| all_name);
                let no_host = *all_no_host || *no_host;
                let no_owner = *all_no_owner || *no_owner;
                let owner = all_owner.clone().map_or(owner, |_| all_owner);
                let path = *all_path || *path;
                let sort_by =
                    all_sort_by.clone().map_or(sort_by, |_| all_sort_by);

                list_unmanaged(
                    *hidden,
                    host.as_ref(),
                    owner.as_ref(),
                    name.as_ref(),
                    me,
                    no_host,
                    no_owner,
                    path,
                    sort_by.as_ref(),
                )
            }

            Some(ListSubcommand::Owners { all, hidden }) => {
                owners(*all, *hidden)
            }
        },

        Some(Command::New { path: _ }) => {
            eprintln!("Implement new!");

            Ok(())
        }

        Some(Command::Remove {
            repos,
            host,
            owner,
            me,
            force,
        }) => remove(repos, host.as_ref(), owner.as_ref(), *me, *force),

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
