use anyhow::Result;
use repo::parse_repo;
use repo::Repo;

use crate::config::get_config;

fn parse_repos(repos: &[String]) -> Vec<Repo> {
    repos
        .iter()
        .filter_map(|repo| parse_repo(repo).ok())
        .collect()
}

fn filter_unique_repos(repos: Vec<Repo>) -> Vec<Repo> {
    let mut repos_to_add: Vec<Repo> = repos
        .clone()
        .into_iter()
        .filter_map(|repo| {
            if repo.path.is_some() {
                Some(repo)
            } else {
                None
            }
        })
        .collect();

    let remote_repos: Vec<Repo> = repos
        .into_iter()
        .filter_map(|repo| {
            if repo.path.is_none() && !repos_to_add.contains(&repo) {
                Some(repo)
            } else {
                None
            }
        })
        .collect();

    repos_to_add.extend(remote_repos);

    repos_to_add
}

pub fn add(repos: &[String]) -> Result<()> {
    if let Some(root_directory) = get_config()?.root_directory {
        for repo in filter_unique_repos(parse_repos(repos)) {
            println!("Adding {} to {}", repo.url, repo.path(&root_directory));
        }
    } else {
        todo!("print error message");
    }

    Ok(())
}

// TODO: test filter_unique_repos
