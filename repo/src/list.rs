use std::path::{Component, Path, PathBuf};
use std::process::Command;

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
                if path.file_type().is_dir() && path.depth() == 3 {
                    path.path().strip_prefix(root_directory).map_or(
                        None,
                        |dir_entry| {
                            filter_path(dir_entry, host, owner, name).map(
                                |path| {
                                    Path::new(root_directory)
                                        .join(path)
                                        .to_string_lossy()
                                        .to_string()
                                },
                            )
                        },
                    )
                } else {
                    None
                }
            })
        })
        .collect()
}

#[must_use]
pub fn get_repos(
    root_directory: &str,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
) -> Vec<Repo> {
    let repo_paths = get_repo_paths(root_directory, host, owner, name);

    let repos: Vec<Repo> = repo_paths
        .iter()
        .filter_map(|repo| Repo::from(repo).ok())
        .collect();

    repos
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
            if Command::new("git")
                .args(["-C", &dir_entry.path().to_string_lossy(), "rev-parse"])
                .status()
                .ok()?
                .success()
            {
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
