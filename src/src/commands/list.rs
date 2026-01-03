use std::collections::HashSet;

use anyhow::Result;
use repo::{
    config::{get_config, get_root_directory},
    list::{
        get_repos, list_all_repos, list_managed_repos, list_non_managed_repos,
        sort_case_insensitive, SortBy,
    },
};

pub fn hosts(all: bool, hidden: bool) -> Result<()> {
    let mut hosts: Vec<String> =
        get_repos(&get_root_directory()?, all, hidden)?
            .into_iter()
            .map(|repo| repo.host)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

    hosts.sort_by(|a, b| sort_case_insensitive(a, b));

    println!("{}", hosts.join("\n"));

    Ok(())
}

pub fn names(all: bool, hidden: bool) -> Result<()> {
    let mut names: Vec<String> =
        get_repos(&get_root_directory()?, all, hidden)?
            .into_iter()
            .map(|repo| repo.name)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

    names.sort_by(|a, b| sort_case_insensitive(a, b));

    println!("{}", names.join("\n"));

    Ok(())
}

pub fn owners(all: bool, hidden: bool) -> Result<()> {
    let mut owners: Vec<String> =
        get_repos(&get_root_directory()?, all, hidden)?
            .into_iter()
            .map(|repo| repo.owner)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

    owners.sort_by(|a, b| sort_case_insensitive(a, b));

    println!("{}", owners.join("\n"));

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
            get_config()?,
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

pub fn list_non_managed(
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
    let repos = list_non_managed_repos(
        get_config()?,
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
        get_config()?,
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
