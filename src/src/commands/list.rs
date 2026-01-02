use std::collections::HashSet;

use anyhow::Result;
use repo::list::{
    get_repos, list_all_repos, list_managed_repos, list_non_managed_repos,
    SortBy,
};

use crate::config::get_root_directory;

pub fn hosts() -> Result<()> {
    let mut hosts: Vec<String> = get_repos(&get_root_directory()?)?
        .into_iter()
        .map(|repo| repo.host)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    hosts.sort();

    println!("{}", hosts.join("\n"));

    Ok(())
}

pub fn names() -> Result<()> {
    let mut hosts: Vec<String> = get_repos(&get_root_directory()?)?
        .into_iter()
        .map(|repo| repo.name)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    hosts.sort();

    println!("{}", hosts.join("\n"));

    Ok(())
}

pub fn owners() -> Result<()> {
    let mut hosts: Vec<String> = get_repos(&get_root_directory()?)?
        .into_iter()
        .map(|repo| repo.owner)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    hosts.sort();

    println!("{}", hosts.join("\n"));

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
    no_host: bool,
    no_owner: bool,
    path: bool,
    sort_by: Option<&SortByOption>,
) -> Result<()> {
    print!(
        "{}",
        list_managed_repos(
            &get_root_directory()?,
            host,
            owner,
            name,
            no_host,
            no_owner,
            path,
            get_sort_by_value(sort_by)
        )
        .join("\n")
    );

    Ok(())
}

pub fn list_non_managed(
    include_hidden: bool,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    no_host: bool,
    no_owner: bool,
    path: bool,
    sort_by: Option<&SortByOption>,
) -> Result<()> {
    let repos = list_non_managed_repos(
        &get_root_directory()?,
        include_hidden,
        host,
        owner,
        name,
        no_host,
        no_owner,
        path,
        get_sort_by_value(sort_by),
    )?;

    print!("{}", repos.join("\n"));

    Ok(())
}

pub fn list_all(
    include_hidden: bool,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    no_host: bool,
    no_owner: bool,
    path: bool,
    sort_by: Option<&SortByOption>,
) -> Result<()> {
    let repos = list_all_repos(
        &get_root_directory()?,
        include_hidden,
        host,
        owner,
        name,
        no_host,
        no_owner,
        path,
        get_sort_by_value(sort_by),
    )?;

    print!("{}", repos.join("\n"));

    Ok(())
}
