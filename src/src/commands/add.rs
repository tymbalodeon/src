use anyhow::Result;
use repo::repo::Repo;

use crate::config::get_config;
use crate::log::{log, LogLevel};

fn parse_repos(repos: &[String]) -> Vec<Repo> {
    repos
        .iter()
        .filter_map(|repo| {
            Repo::from(repo).map_or_else(
                |_| {
                    log(&LogLevel::Error, &format!("invalid path: {repo:?}"));

                    None
                },
                Some,
            )
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
                let repo = repo.path.map_or(Some(repo.url), |path| {
                    // TODO: print error if this fails
                    path.canonicalize().map_or(None, |path| {
                        Some(path.to_string_lossy().to_string())
                    })
                });

                if let Some(repo) = repo {
                    println!("Adding {repo} to {path}");
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn it_prefers_local_paths_to_remote_urls() {
        let local_repo = Repo::new(
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
            Repo::new(
                "github.com",
                "src",
                "tymbalodeon",
                None,
                "git@github.com:tymbalodeon/src.git",
            ),
        ];

        let repos_with_local_second = vec![
            Repo::new(
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
