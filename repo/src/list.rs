use std::path::{Component, Path, PathBuf};
use std::process::Command;

use dirs::home_dir;
use walkdir::{DirEntry, WalkDir};

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

fn filter_path(
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

// TODO: use this in the src binary, so that it can print a warning about not
// having git installed on error
//
/// # Errors
///
/// Will return `RepoError` if it git is not installed
pub fn filter_git_repos(
    paths: &[DirEntry],
) -> Result<Vec<DirEntry>, RepoError> {
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
                    if dir_entry.file_type().is_dir() && dir_entry.depth() == 3
                    {
                        dir_entry.path().strip_prefix(root_directory).map_or(
                            None,
                            |dir_entry| {
                                let components: Vec<Component> =
                                    dir_entry.components().collect();

                                let mut repo = filter_path(
                                    Some(&dir_entry.to_path_buf()),
                                    &components,
                                    host,
                                    0,
                                );

                                repo = filter_path(
                                    repo.as_ref(),
                                    &components,
                                    owner,
                                    1,
                                );

                                repo = filter_path(
                                    repo.as_ref(),
                                    &components,
                                    name,
                                    2,
                                );

                                repo.map(|path| {
                                    Path::new(root_directory)
                                        .join(path)
                                        .to_string_lossy()
                                        .to_string()
                                })
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
