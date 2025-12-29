use std::path::{Component, Path, PathBuf};
use std::process::Command;

use dirs::home_dir;
use walkdir::{DirEntry, WalkDir};

use crate::error::SrcRepoError;
use crate::repo::Repo;

fn get_repo_paths(
    root_directory: &str,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
) -> Result<Vec<String>, SrcRepoError> {
    Ok(
        WalkDir::new(home_dir().ok_or(SrcRepoError::HomeDir)?.join("src"))
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
            .collect(),
    )
}

/// # Errors
///
/// Will return `SrcRepoError` if it cannot determine the $HOME directory
pub fn get_repos(
    root_directory: &str,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
) -> Result<Vec<Repo>, SrcRepoError> {
    let repo_paths = get_repo_paths(root_directory, host, owner, name)?;

    let repos: Vec<Repo> = repo_paths
        .iter()
        .filter_map(|repo| Repo::from(repo).ok())
        .collect();

    Ok(repos)
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

/// # Errors
///
/// Will return `SrcRepoError` if it cannot determine $HOME directory
pub fn list_repos(
    root_directory: &str,
    host: Option<&String>,
    owner: Option<&String>,
    name: Option<&String>,
    no_host: bool,
    no_owner: bool,
    path: bool,
) -> Result<Vec<String>, SrcRepoError> {
    Ok(format_repo_list(
        &get_repo_paths(root_directory, host, owner, name)?,
        no_host,
        no_owner,
        path,
    ))
}
