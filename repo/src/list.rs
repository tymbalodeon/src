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
    no_host: bool,
    no_owner: bool,
    as_path: bool,
) -> Result<Vec<String>, RepoError> {
    let repos =
        WalkDir::new(home_dir().ok_or(RepoError::RepoPath)?.join("src"))
            .into_iter()
            .filter_map(|path| {
                path.as_ref().map_or(None, |path| {
                    if path.file_type().is_dir() && path.depth() == 3 {
                        path.path().to_str().map(std::string::ToString::to_string)
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<String>>();

    Ok(format_repo_list(&repos, no_host, no_owner, as_path))
}
