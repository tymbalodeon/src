use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;

use dirs::home_dir;
use rust_fuzzy_search::fuzzy_search_threshold;
use walkdir::{DirEntry, WalkDir};

use crate::error::SrcRepoError;
use crate::repo::Repo;

#[must_use]
pub fn get_managed_repo_paths(root_directory: &str) -> Vec<String> {
    WalkDir::new(root_directory)
        .into_iter()
        .filter_map(|path| {
            path.as_ref().map_or(None, |path| {
                if !path.file_type().is_dir() || path.depth() != 3 {
                    return None;
                }

                path.path().strip_prefix(root_directory).map_or(
                    None,
                    |dir_entry| {
                        Some(
                            PathBuf::from(root_directory)
                                .join(dir_entry)
                                .to_string_lossy()
                                .to_string(),
                        )
                    },
                )
            })
        })
        .collect()
}

/// # Errors
///
/// Will return `SrcRepoError` if it fails to parse a repo in `root_directory`
pub fn get_repos(
    root_directory: &str,
    all: bool,
    hidden: bool,
) -> Result<Vec<Repo>, SrcRepoError> {
    let paths = if all {
        get_repo_paths(None, hidden)?
    } else {
        get_managed_repo_paths(root_directory)
    };

    paths.iter().map(|repo| Repo::from(repo)).collect()
}

fn is_git_repo(path: &DirEntry) -> bool {
    if let Ok(result) = Command::new("git")
        .args(["-C", &path.path().to_string_lossy(), "rev-parse"])
        .output()
    {
        result.status.success()
    } else {
        false
    }
}

// TODO: use this in the src binary, so that it can print a warning about not
// having git installed on error
//
/// # Errors
///
/// Will return `SrcRepoError` if it git is not installed
pub fn filter_git_repos(
    paths: Vec<DirEntry>,
) -> Result<Vec<DirEntry>, SrcRepoError> {
    Ok(paths
        .into_iter()
        .filter_map(|dir_entry| {
            if is_git_repo(&dir_entry) {
                Some(dir_entry)
            } else {
                None
            }
        })
        .collect())
}

fn sort_case_insensitive(a: &str, b: &str) -> std::cmp::Ordering {
    a.to_lowercase().cmp(&b.to_lowercase())
}

pub enum SortBy {
    Host,
    Name,
    Owner,
}

const FUZZY_SEARCH_THRESHOLD: f32 = 0.1;

fn list_repos(
    root_directory: &str,
    repo_paths: &[String],
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    no_host: bool,
    no_owner: bool,
    path: bool,
    unique: bool,
    sort_by: Option<&SortBy>,
) -> Vec<String> {
    let mut repos: Vec<Repo> = repo_paths
        .iter()
        .filter_map(|path| Repo::from(path).ok())
        .collect();

    if let Some(host) = host {
        let hosts: Vec<&str> =
            repos.iter().map(|repo| repo.host.as_str()).collect();

        repos = fuzzy_search_threshold(host, &hosts, FUZZY_SEARCH_THRESHOLD)
            .iter()
            .map(|item| item.0)
            .collect::<HashSet<&str>>()
            .iter()
            .flat_map(|host| {
                repos
                    .iter()
                    .filter(|&repo| repo.host == *host)
                    .cloned()
                    .collect::<Vec<Repo>>()
            })
            .collect();
    }

    if let Some(owner) = owner {
        let owners: Vec<&str> =
            repos.iter().map(|repo| repo.owner.as_str()).collect();

        repos = fuzzy_search_threshold(owner, &owners, FUZZY_SEARCH_THRESHOLD)
            .iter()
            .map(|item| item.0)
            .collect::<HashSet<&str>>()
            .iter()
            .flat_map(|owner| {
                repos
                    .iter()
                    .filter(|&repo| repo.owner == *owner)
                    .cloned()
                    .collect::<Vec<Repo>>()
            })
            .collect();
    }

    if let Some(name) = name {
        let names: Vec<&str> =
            repos.iter().map(|repo| repo.name.as_str()).collect();

        repos = fuzzy_search_threshold(name, &names, FUZZY_SEARCH_THRESHOLD)
            .iter()
            .map(|item| item.0)
            .collect::<HashSet<&str>>()
            .iter()
            .flat_map(|name| {
                repos
                    .iter()
                    .filter(|&repo| repo.name == *name)
                    .cloned()
                    .collect::<Vec<Repo>>()
            })
            .collect();
    }

    if let Some(sort_by) = &sort_by {
        match sort_by {
            SortBy::Host => repos.sort_by(|a, b| a.host.cmp(&b.host)),
            SortBy::Name => repos.sort_by(|a, b| a.name.cmp(&b.name)),
            SortBy::Owner => repos.sort_by(|a, b| a.owner.cmp(&b.owner)),
        }
    }

    let mut formatted_repos: Vec<String> = repos
        .iter()
        .filter_map(|repo| {
            if path {
                if let Some(path) = &repo.local_source_path {
                    Some(path.to_string_lossy().to_string())
                } else {
                    repo.managed_path(root_directory).ok()
                }
            } else {
                Some(repo.display(no_host, no_owner))
            }
        })
        .collect();

    if unique {
        formatted_repos = formatted_repos
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
    }

    if sort_by.is_none() {
        formatted_repos
            .sort_by(|a: &String, b: &String| sort_case_insensitive(a, b));
    }

    formatted_repos
}

#[must_use]
pub fn list_managed_repos(
    root_directory: &str,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    no_host: bool,
    no_owner: bool,
    path: bool,
    sort_by: Option<&SortBy>,
) -> Vec<String> {
    list_repos(
        root_directory,
        &get_managed_repo_paths(root_directory),
        host,
        owner,
        name,
        no_host,
        no_owner,
        path,
        false,
        sort_by,
    )
}

fn is_managed_path(path: &DirEntry, root_directory: Option<&str>) -> bool {
    root_directory.is_some_and(|root_directory| {
        path.depth() == 4 && path.path().starts_with(root_directory)
    })
}

/// # Errors
///
/// Will return `SrcRepoError` if it fails to determine the `$HOME` directory
pub fn get_repo_paths(
    root_directory: Option<&str>,
    hidden: bool,
) -> Result<Vec<String>, SrcRepoError> {
    let home_dir = home_dir().ok_or(SrcRepoError::HomeDir)?;

    Ok(WalkDir::new(&home_dir)
        .into_iter()
        .filter_map(|path| {
            path.as_ref().map_or(None, |path| {
                if !path.file_type().is_dir()
                    || !path.path().join(".git").exists()
                    || is_managed_path(path, root_directory)
                    || (!hidden
                        && path
                            .path()
                            .strip_prefix(&home_dir)
                            .ok()?
                            .to_string_lossy()
                            .to_string()
                            .starts_with('.'))
                    || !is_git_repo(path)
                {
                    return None;
                }

                root_directory.map_or_else(
                    || Some(path.path().to_string_lossy().to_string()),
                    |root_directory| {
                        path.path().strip_prefix(root_directory).map_or(
                            None,
                            |dir_entry| {
                                Some(
                                    PathBuf::from(root_directory)
                                        .join(dir_entry)
                                        .to_string_lossy()
                                        .to_string(),
                                )
                            },
                        )
                    },
                )
            })
        })
        .collect())
}

/// # Errors
///
/// Will return `SrcRepoError` if it fails to determine the `$HOME` directory
pub fn list_non_managed_repos(
    root_directory: &str,
    hidden: bool,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    no_host: bool,
    no_owner: bool,
    path: bool,
    sort_by: Option<&SortBy>,
) -> Result<Vec<String>, SrcRepoError> {
    Ok(list_repos(
        root_directory,
        &get_repo_paths(Some(root_directory), hidden)?,
        host,
        owner,
        name,
        no_host,
        no_owner,
        path,
        true,
        sort_by,
    ))
}

/// # Errors
///
/// Will return `SrcRepoError` if it fails to determine the `$HOME` directory
pub fn list_all_repos(
    root_directory: &str,
    hidden: bool,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    no_host: bool,
    no_owner: bool,
    path: bool,
    sort_by: Option<&SortBy>,
) -> Result<Vec<String>, SrcRepoError> {
    Ok(list_repos(
        root_directory,
        &get_repo_paths(None, hidden)?,
        host,
        owner,
        name,
        no_host,
        no_owner,
        path,
        true,
        sort_by,
    ))
}
