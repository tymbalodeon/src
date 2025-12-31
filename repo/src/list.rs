use std::path::{Component, Path, PathBuf};
use std::process::Command;

use dirs::home_dir;
use walkdir::{DirEntry, WalkDir};

use crate::error::SrcRepoError;
use crate::repo::Repo;

fn filter_path_by_component(
    path: Option<&PathBuf>,
    components: &[Component],
    filter: Option<&String>,
    index: usize,
) -> Option<PathBuf> {
    filter
        .map_or_else(
            || path,
            |filter| {
                if &(components[index]
                    .as_os_str()
                    .to_string_lossy()
                    .to_string())
                    == filter
                {
                    path
                } else {
                    None
                }
            },
        )
        .cloned()
}

fn filter_path(
    path: &Path,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
) -> Option<PathBuf> {
    let components: Vec<Component> = path.components().collect();

    let mut repo = filter_path_by_component(
        Some(&path.to_path_buf()),
        &components,
        host,
        0,
    );

    repo = filter_path_by_component(repo.as_ref(), &components, owner, 1);

    filter_path_by_component(repo.as_ref(), &components, name, 2)
}

// TODO: test  me
fn get_repo_path(
    path: &DirEntry,
    root_directory: &str,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
) -> Option<String> {
    if !path.file_type().is_dir() || path.depth() != 3 {
        return None;
    }

    path.path()
        .strip_prefix(root_directory)
        .map_or(None, |dir_entry| {
            filter_path(dir_entry, host, owner, name).map(|path| {
                PathBuf::from(root_directory)
                    .join(path)
                    .to_string_lossy()
                    .to_string()
            })
        })
}

#[must_use]
pub fn get_repo_paths(
    root_directory: &str,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
) -> Vec<String> {
    WalkDir::new(root_directory)
        .into_iter()
        .filter_map(|path| {
            path.as_ref().map_or(None, |path| {
                get_repo_path(path, root_directory, host, owner, name)
            })
        })
        .collect()
}

// TODO: add ability to ignore hidden folders
// TODO: add equivalent for repos and use repos by default, then --path for this
// TODO: check for nested git repos within the src managed folders (git repos
// NOT at depth 3)
#[must_use]
pub fn get_non_managed_repo_paths(root_directory: &str) -> Vec<String> {
    WalkDir::new(home_dir().unwrap())
        .into_iter()
        .filter_map(|path| {
            path.as_ref().map_or(None, |path| {
                if path.file_type().is_dir()
                    && !path.path().starts_with(root_directory)
                    && path.path().join(".git").exists()
                    && is_git_repo(path)
                {
                    Some(path.path().to_string_lossy().to_string())
                } else {
                    None
                }
            })
        })
        .collect()
}

/// # Errors
///
/// Will return `SrcRepoError` if it fails to parse a repo in `root_directory`
pub fn get_repos(
    root_directory: &str,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
) -> Result<Vec<Repo>, SrcRepoError> {
    get_repo_paths(root_directory, host, owner, name)
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
    paths: &[DirEntry],
) -> Result<Vec<DirEntry>, SrcRepoError> {
    Ok(paths
        .iter()
        .filter_map(|dir_entry| {
            if is_git_repo(dir_entry) {
                Some(dir_entry.clone())
            } else {
                None
            }
        })
        .collect())
}

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
    let mut repos: Vec<String> =
        get_repo_paths(root_directory, host, owner, name)
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
