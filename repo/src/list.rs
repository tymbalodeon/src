use std::path::{Component, Path};

use dirs::home_dir;
use walkdir::WalkDir;

use crate::repo::{Repo, RepoError};

fn format_repo_list(
    repos: &[String],
    no_host: bool,
    no_owner: bool,
    path: bool,
) -> Vec<String> {
    let mut repos: Vec<String> = repos
        .iter()
        .filter_map(|repo| {
            if path {
                Some(repo.clone())
            } else if let Ok(repo) = Repo::from(repo) {
                Some(repo.display(no_host, no_owner))
            } else {
                None
            }
        })
        .collect();

    repos.sort();

    repos
}

/// # Errors
///
/// Will return `RepoError` if it cannot determine $HOME
pub fn list_repos(
    root_directory: &str,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    no_host: bool,
    no_owner: bool,
    path: bool,
) -> Result<Vec<String>, RepoError> {
    let repos =
        WalkDir::new(home_dir().ok_or(RepoError::RepoPath)?.join("src"))
            .into_iter()
            .filter_map(|dir_entry| {
                dir_entry.as_ref().map_or(None, |dir_entry| {
                    // TODO: Check if is git repository as well
                    if dir_entry.file_type().is_dir() && dir_entry.depth() == 3
                    {
                        dir_entry.path().strip_prefix(root_directory).map_or(
                            None,
                            |dir_entry| {
                                let components: Vec<Component> =
                                    dir_entry.components().collect();

                                let mut repo =
                                    host.map_or(Some(dir_entry), |host| {
                                        if &(components[0]
                                            .as_os_str()
                                            .to_string_lossy()
                                            .to_string())
                                            == host
                                        {
                                            Some(dir_entry)
                                        } else {
                                            None
                                        }
                                    });

                                repo = if repo.is_some() {
                                    owner.map_or(Some(dir_entry), |owner| {
                                        if &(components[1]
                                            .as_os_str()
                                            .to_string_lossy()
                                            .to_string())
                                            == owner
                                        {
                                            Some(dir_entry)
                                        } else {
                                            None
                                        }
                                    })
                                } else {
                                    None
                                };

                                repo = if repo.is_some() {
                                    name.map_or(Some(dir_entry), |name| {
                                        if &(components[2]
                                            .as_os_str()
                                            .to_string_lossy()
                                            .to_string())
                                            == name
                                        {
                                            Some(dir_entry)
                                        } else {
                                            None
                                        }
                                    })
                                } else {
                                    None
                                };

                                repo.map(|path| Path::new(root_directory)
                                            .join(path)
                                            .to_string_lossy()
                                            .to_string())
                            },
                        )
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<String>>();

    Ok(format_repo_list(&repos, no_host, no_owner, path))
}
