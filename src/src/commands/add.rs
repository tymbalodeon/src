use anyhow::Result;
use repo::parse_repo;
use repo::Repo;

use crate::config::get_config;

fn parse_repos(repos: &[String]) -> Vec<Repo> {
    repos
        .iter()
        .filter_map(|repo| {
            if let Ok(repo) = parse_repo(repo) {
                Some(repo)
            } else {
                // TODO: add better error message
                eprintln!("invalid path: {repo:?}");

                None
            }
        })
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
            if let Ok(path) = repo.path(&root_directory) {
                println!("Adding {} to {}", repo.url, path);
            }
        }
    } else {
        todo!("print error message");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn it_prefers_local_paths_to_remote_urls() {
        let local_repo = Repo::from(
            "github.com",
            "src",
            "tymbalodeon",
            Some(PathBuf::from(
                "/home/benrosen/src/github.com/tymbalodeon/src",
            )),
            "git@github.com:tymbalodeon/src.git",
        );

        let repos_with_local_first = vec![
            local_repo.clone(),
            Repo::from(
                "github.com",
                "src",
                "tymbalodeon",
                None,
                "git@github.com:tymbalodeon/src.git",
            ),
        ];

        let repos_with_local_second = vec![
            Repo::from(
                "github.com",
                "src",
                "tymbalodeon",
                None,
                "git@github.com:tymbalodeon/src.git",
            ),
            local_repo.clone(),
        ];

        assert_eq!(
            filter_unique_repos(repos_with_local_first),
            vec![local_repo.clone()]
        );

        assert_eq!(
            filter_unique_repos(repos_with_local_second),
            vec![local_repo]
        );
    }
}
