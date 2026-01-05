use std::path::Path;
use std::process::Command;

use anyhow::Result;
use repo::config::get_config;
use repo::repo::Repo;
use repo::{config::get_root_directory, list::get_managed_repo_paths};

use crate::repo::parse_repos_with_error_log;

fn filter_unique_repos(repos: &[Repo]) -> Vec<Repo> {
    let mut repos_to_add: Vec<Repo> = repos
        .iter()
        .filter_map(|repo| {
            if repo.local_source_path.is_some() {
                Some(repo.clone())
            } else {
                None
            }
        })
        .collect();

    let remote_repos: Vec<Repo> = repos
        .iter()
        .filter_map(|repo| {
            if repo.local_source_path.is_none() && !repos_to_add.contains(repo)
            {
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
    let repos = parse_repos_with_error_log(&get_config()?, repos, false);
    let root_directory = get_root_directory()?;
    let repo_paths = get_managed_repo_paths(&root_directory);

    for repo in filter_unique_repos(&repos) {
        if let Ok(managed_path) = repo.managed_path(&root_directory) {
            if let Some(ref local_source_path) = repo.local_source_path {
                if force || !repo_paths.contains(&managed_path) {
                    println!(
                        "Moving {} to {managed_path}",
                        local_source_path.to_string_lossy()
                    );

                    Command::new("mv")
                        .args(vec![
                            &local_source_path.to_string_lossy().to_string(),
                            &format!(
                                "{root_directory}/{}/{}",
                                repo.host, repo.owner
                            ),
                        ])
                        .status()?;
                }
            } else {
                let managed_path = repo.managed_path(&root_directory)?;

                if force || !repo_paths.contains(&managed_path) {
                    let managed_path = repo.managed_path(&root_directory)?;

                    if force && Path::new(&managed_path).exists() {
                        Command::new("rm")
                            .args(vec![
                                "--force",
                                "--recursive",
                                &managed_path,
                            ])
                            .status()?;
                    }

                    Command::new("git")
                        .args(vec!["clone", &repo.url(), &managed_path])
                        .status()?;
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
            "tymbalodeon",
            "src",
            Some(PathBuf::from(
                "/home/benrosen/src/github.com/tymbalodeon/src",
            )),
            "git@github.com:tymbalodeon/src.git",
        );

        let repos_with_local_first = vec![
            local_repo.clone(),
            Repo::new(
                "github.com",
                "tymbalodeon",
                "src",
                None,
                "git@github.com:tymbalodeon/src.git",
            ),
        ];

        let repos_with_local_second = vec![
            Repo::new(
                "github.com",
                "tymbalodeon",
                "src",
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
