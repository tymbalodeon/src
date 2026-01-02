use std::collections::HashSet;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use dirs::home_dir;
use rust_fuzzy_search::fuzzy_search_threshold;
use walkdir::{DirEntry, WalkDir};

use crate::error::SrcRepoError;
use crate::repo::Repo;

fn filter_path_by_component(
    path: Option<PathBuf>,
    components: &[Component],
    filter: Option<&String>,
    index: usize,
) -> Option<PathBuf> {
    match filter {
        Some(filter) => {
            if &(components[index].as_os_str().to_string_lossy().to_string())
                == filter
            {
                path
            } else {
                None
            }
        }
        None => path,
    }
}

fn filter_path(
    path: &Path,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
) -> Option<PathBuf> {
    let components: Vec<Component> = path.components().collect();

    let mut repo = filter_path_by_component(
        Some(path.to_path_buf()),
        &components,
        host,
        0,
    );

    repo = filter_path_by_component(repo, &components, owner, 1);

    filter_path_by_component(repo, &components, name, 2)
}

#[must_use]
pub fn get_repo_paths(root_directory: &str) -> Vec<String> {
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
    // host: Option<&String>,
    // owner: Option<&String>,
    // name: Option<&String>,
) -> Result<Vec<Repo>, SrcRepoError> {
    // TODO: add back filters?
    get_repo_paths(root_directory)
        .iter()
        .map(|repo| Repo::from(repo))
        .collect()
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

const FUZZY_SEARCH_THRESHOLD: f32 = 0.1;

#[must_use]
pub fn list_repos(
    root_directory: &str,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    no_host: bool,
    no_owner: bool,
    path: bool,
) -> Vec<String> {
    // FIXME
    let repo_paths = get_repo_paths(root_directory);

    let mut repos: Vec<Repo> = repo_paths
        .iter()
        .filter_map(|path| Repo::from(path).ok())
        .collect();

    if let Some(host) = host {
        let hosts: Vec<&str> =
            repos.iter().map(|repo| repo.host.as_str()).collect();

        repos = fuzzy_search_threshold(&host, &hosts, FUZZY_SEARCH_THRESHOLD)
            .iter()
            .map(|item| item.0)
            .collect::<HashSet<&str>>()
            .iter()
            .flat_map(|host| {
                repos
                    .iter()
                    .cloned()
                    .filter(|repo| repo.host == *host)
                    .collect::<Vec<Repo>>()
            })
            .collect()
    }

    if let Some(owner) = owner {
        let owners: Vec<&str> =
            repos.iter().map(|repo| repo.owner.as_str()).collect();

        repos = fuzzy_search_threshold(&owner, &owners, FUZZY_SEARCH_THRESHOLD)
            .iter()
            .map(|item| item.0)
            .collect::<HashSet<&str>>()
            .iter()
            .flat_map(|owner| {
                repos
                    .iter()
                    .cloned()
                    .filter(|repo| repo.owner == *owner)
                    .collect::<Vec<Repo>>()
            })
            .collect()
    }

    if let Some(name) = name {
        let names: Vec<&str> =
            repos.iter().map(|repo| repo.name.as_str()).collect();

        repos = fuzzy_search_threshold(&name, &names, FUZZY_SEARCH_THRESHOLD)
            .iter()
            .map(|item| item.0)
            .collect::<HashSet<&str>>()
            .iter()
            .flat_map(|name| {
                repos
                    .iter()
                    .cloned()
                    .filter(|repo| repo.name == *name)
                    .collect::<Vec<Repo>>()
            })
            .collect()
    }

    let mut formatted_repos: Vec<String> = repos
        .iter()
        .filter_map(|repo| {
            if path {
                repo.path(root_directory).ok()
            } else {
                Some(repo.display(no_host, no_owner))
            }
        })
        .collect();

    formatted_repos
        .sort_by(|a: &String, b: &String| sort_case_insensitive(a, b));

    formatted_repos
}

/// # Errors
///
/// Will return `SrcRepoError` if it fails to determine the `$HOME` directory
pub fn get_non_managed_repo_paths(
    root_directory: &str,
    include_hidden: bool,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
) -> Result<Vec<String>, SrcRepoError> {
    let home_dir = home_dir().ok_or(SrcRepoError::HomeDir)?;

    Ok(WalkDir::new(&home_dir)
        .into_iter()
        .filter_map(|path| {
            path.as_ref().map_or(None, |path| {
                if path.file_type().is_dir()
                    && path.path().join(".git").exists()
                    && (!path.path().starts_with(root_directory)
                        || path.depth() != 4)
                    && (!path
                        .path()
                        .strip_prefix(&home_dir)
                        .ok()?
                        .to_string_lossy()
                        .to_string()
                        .starts_with('.')
                        || include_hidden)
                    && is_git_repo(path)
                {
                    filter_path(
                        Path::new(
                            &Repo::from(&path.path().to_string_lossy())
                                .ok()?
                                .path(root_directory)
                                .ok()?
                                .strip_prefix(&format!("{root_directory}/"))?,
                        ),
                        host,
                        owner,
                        name,
                    )
                    .map(|_| path.path().to_string_lossy().to_string())
                } else {
                    None
                }
            })
        })
        .collect())
}

/// # Errors
///
/// Will return `SrcRepoError` if it fails to determine the `$HOME` directory
pub fn list_non_managed_repos(
    root_directory: &str,
    include_hidden: bool,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    no_host: bool,
    no_owner: bool,
    path: bool,
) -> Result<Vec<String>, SrcRepoError> {
    let mut repos: Vec<String> = get_non_managed_repo_paths(
        root_directory,
        include_hidden,
        host,
        owner,
        name,
    )?
    .into_iter()
    .filter_map(|repo| {
        if path {
            Some(repo)
        } else if let Ok(repo) = Repo::from(&repo) {
            Some(repo.display(no_host, no_owner))
        } else {
            None
        }
    })
    .collect();

    if !path {
        repos = repos
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
    }

    repos.sort_by(|a: &String, b: &String| sort_case_insensitive(a, b));

    Ok(repos)
}

/// # Errors
///
/// Will return `SrcRepoError` if it fails to determine the `$HOME` directory
pub fn list_all_repos(
    root_directory: &str,
    include_hidden: bool,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    no_host: bool,
    no_owner: bool,
    path: bool,
) -> Result<Vec<String>, SrcRepoError> {
    let mut repos: Vec<String>;

    repos =
        list_repos(root_directory, host, owner, name, no_host, no_owner, path);

    repos.append(&mut list_non_managed_repos(
        root_directory,
        include_hidden,
        host,
        owner,
        name,
        no_host,
        no_owner,
        path,
    )?);

    repos.sort_by(|a: &String, b: &String| sort_case_insensitive(a, b));

    Ok(repos)
}
