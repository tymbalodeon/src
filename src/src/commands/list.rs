use std::{collections::HashSet, path::PathBuf};

use anyhow::Result;
use repo::{
    config::{Config, get_config, get_root_directory, get_username},
    list::{
        SortBy, get_repos, list_all_repos, list_managed_repos,
        list_unmanaged_repos, sort_case_insensitive,
    },
};

pub fn get_host_names(
    config: Option<&Config>,
    all: bool,
    hidden: bool,
) -> Result<Vec<String>> {
    Ok(get_repos(&get_root_directory(config)?, all, hidden)?
        .into_iter()
        .map(|repo| repo.host)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect())
}

pub fn hosts(config: Option<&Config>, all: bool, hidden: bool) -> Result<()> {
    let mut hosts = get_host_names(config, all, hidden)?;

    hosts.sort_by(|a, b| sort_case_insensitive(a, b));

    println!("{}", hosts.join("\n"));

    Ok(())
}

pub fn get_owner_names(
    config: Option<&Config>,
    all: bool,
    hidden: bool,
) -> Result<Vec<String>> {
    Ok(get_repos(&get_root_directory(config)?, all, hidden)?
        .into_iter()
        .map(|repo| repo.owner)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect())
}

pub fn owners(config: Option<&Config>, all: bool, hidden: bool) -> Result<()> {
    let mut owners = get_owner_names(config, all, hidden)?;

    owners.sort_by(|a, b| sort_case_insensitive(a, b));

    println!("{}", owners.join("\n"));

    Ok(())
}

pub fn names(
    config: Option<&Config>,
    all: bool,
    hidden: bool,
    me: bool,
) -> Result<()> {
    let mut names: Vec<String> =
        get_repos(&get_root_directory(config)?, all, hidden)?
            .into_iter()
            .filter_map(|repo| {
                if me {
                    if repo.owner == get_username(config).ok()? {
                        Some(repo.name)
                    } else {
                        None
                    }
                } else {
                    Some(repo.name)
                }
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

    names.sort_by(|a, b| sort_case_insensitive(a, b));

    println!("{}", names.join("\n"));

    Ok(())
}

#[derive(clap::ValueEnum, Clone)]
pub enum SortByOption {
    Host,
    Name,
    Owner,
}

const fn get_sort_by_value(sort_by: Option<&SortByOption>) -> Option<SortBy> {
    match sort_by {
        Some(sort_by) => match sort_by {
            SortByOption::Host => Some(SortBy::Host),
            SortByOption::Name => Some(SortBy::Name),
            SortByOption::Owner => Some(SortBy::Owner),
        },

        None => None,
    }
}

pub fn list(
    config_file: Option<&PathBuf>,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    me: bool,
    no_host: bool,
    no_owner: bool,
    path: bool,
    sort_by: Option<&SortByOption>,
) -> Result<()> {
    print!(
        "{}",
        list_managed_repos(
            &get_config(config_file)?,
            host,
            owner,
            name,
            me,
            no_host,
            no_owner,
            path,
            get_sort_by_value(sort_by).as_ref()
        )?
        .join("\n")
    );

    Ok(())
}

pub fn list_unmanaged(
    config_file: Option<&PathBuf>,
    hidden: bool,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    me: bool,
    no_host: bool,
    no_owner: bool,
    path: bool,
    sort_by: Option<&SortByOption>,
) -> Result<()> {
    let repos = list_unmanaged_repos(
        &get_config(config_file)?,
        hidden,
        host,
        owner,
        name,
        me,
        no_host,
        no_owner,
        path,
        get_sort_by_value(sort_by).as_ref(),
    )?;

    print!("{}", repos.join("\n"));

    Ok(())
}

pub fn list_all(
    config_file: Option<&PathBuf>,
    hidden: bool,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    me: bool,
    no_host: bool,
    no_owner: bool,
    path: bool,
    sort_by: Option<&SortByOption>,
) -> Result<()> {
    let repos = list_all_repos(
        &get_config(config_file)?,
        hidden,
        host,
        owner,
        name,
        me,
        no_host,
        no_owner,
        path,
        get_sort_by_value(sort_by).as_ref(),
    )?;

    print!("{}", repos.join("\n"));

    Ok(())
}
