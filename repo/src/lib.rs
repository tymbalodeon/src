pub mod list;
pub mod repo;

use repo::{get_local_repo_path, get_url, parse_url, Repo, RepoError};

/// # Errors
///
/// Will return `RepoError` if it cannot determine a git url from `repo`
pub fn parse_repo(repo: &str) -> Result<Repo, RepoError> {
    let local_repo_path = get_local_repo_path(repo);

    parse_url(
        &get_url(repo, local_repo_path.as_ref())?,
        local_repo_path.as_ref(),
    )
}
