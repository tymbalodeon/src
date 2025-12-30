use std::path::PathBuf;

use anyhow::Result;
use repo::list::get_repo_paths;
use repo::repo::Repo;

use crate::config::get_root_directory;
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

fn filter_unique_repos(repos: &[Repo]) -> Vec<Repo> {
    let mut repos_to_add: Vec<Repo> = repos
        .iter()
        .filter_map(|repo| {
            if repo.path.is_some() {
                Some(repo.clone())
            } else {
                None
            }
        })
        .collect();

    let remote_repos: Vec<Repo> = repos
        .iter()
        .filter_map(|repo| {
            if repo.path.is_none() && !repos_to_add.contains(repo) {
                Some(repo.clone())
            } else {
                None
            }
        })
        .collect();

    repos_to_add.extend(remote_repos);

    repos_to_add
}

pub fn add(repos: &[String], force: bool) -> Result<()> {
    let root_directory = get_root_directory()?;
    let repo_paths = get_repo_paths(&root_directory, None, None, None)?;
    let root_directory = PathBuf::from(root_directory);

    for repo in filter_unique_repos(&parse_repos(repos)) {
        if let Ok(path) = repo.path(&root_directory) {
            let repo = repo.path.map_or(Some(repo.url), |path| {
                // TODO: print error if this fails
                path.canonicalize().map_or(None, |path| {
                    Some(path.to_string_lossy().to_string())
                })
            });

            if let Some(repo) = repo
                && (force || !repo_paths.contains(&repo)) {
                    println!("Adding {repo} to {path}");
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
            filter_unique_repos(&repos_with_local_first),
            vec![local_repo.clone()]
        );

        assert_eq!(
            filter_unique_repos(&repos_with_local_second),
            vec![local_repo]
        );
    }
}
