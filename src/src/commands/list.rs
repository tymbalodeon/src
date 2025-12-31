use std::collections::HashSet;

use anyhow::Result;
use repo::list::{get_non_managed_repo_paths, get_repos, list_repos};

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
    non_managed: bool,
    path: bool,
    sort_by: Option<&SortByComponent>,
) -> Result<()> {
    if sort_by.is_some() {
        println!("Implement me!");
    }

    let repos = if non_managed {
        println!("Implement me!");

        get_non_managed_repo_paths(&get_root_directory()?)
    } else {
        list_repos(
            &get_root_directory()?,
            host,
            owner,
            name,
            no_host,
            no_owner,
            path,
        )
    };

    Ok(print!("{}", repos.join("\n")))
}
