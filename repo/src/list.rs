use dirs::home_dir;
use walkdir::WalkDir;

use crate::repo::{Repo, RepoError};

/// # Errors
///
/// Will return `RepoError` if it cannot determine $HOME
pub fn list(as_path: bool) -> Result<Vec<String>, RepoError> {
    let mut repos =
        WalkDir::new(home_dir().ok_or(RepoError::RepoPath)?.join("src"))
            .into_iter()
            .filter_map(|path| {
                path.as_ref().map_or(None, |path| {
                    if path.file_type().is_dir() && path.depth() == 3 {
                        path.path().to_str().and_then(|path| {
                            if as_path {
                                Some(path.to_string())
                            } else if let Ok(repo) = Repo::from(path) {
                                Some(format!("{repo}"))
                            } else {
                                None
                            }
                        })
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<String>>();

    repos.sort();

    Ok(repos)
}
