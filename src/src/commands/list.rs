use std::collections::HashSet;

use anyhow::Result;
use repo::list::{
    get_repos, list_all_repos, list_non_managed_repos, list_repos,
};

use crate::config::get_root_directory;

pub fn hosts() -> Result<()> {
    let mut hosts: Vec<String> =
        get_repos(&get_root_directory()?, None, None, None)?
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
    let mut hosts: Vec<String> =
        get_repos(&get_root_directory()?, None, None, None)?
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
    let mut hosts: Vec<String> =
        get_repos(&get_root_directory()?, None, None, None)?
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
pub enum SortByComponent {
    Host,
    Name,
    Owner,
}

pub fn list(
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    no_host: bool,
    no_owner: bool,
    path: bool,
    sort_by: Option<&SortByComponent>,
) -> Result<()> {
    if sort_by.is_some() {
        println!("Implement me!");
    }

    println!(
        "{}",
        list_repos(
            &get_root_directory()?,
            host,
            owner,
            name,
            no_host,
            no_owner,
            path,
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
    )?;

    print!("{}", repos.join("\n"));

    Ok(())
}
