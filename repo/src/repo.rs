use std::fmt;
use std::path::{Path, PathBuf};

use derivative::Derivative;
use git2::Repository;
use git_url_parse::types::provider::GenericProvider;
use git_url_parse::GitUrl;
use shellexpand::tilde;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("failed to read git directory")]
    Git(#[from] git2::Error),

    #[error("failed to get remote \"origin\"")]
    GitUrl,

    #[error("failed to parse git url")]
    GitUrlParseError(#[from] git_url_parse::GitUrlParseError),

    #[error("invalid characters in repo path")]
    RepoPath,
}

#[derive(Clone, Debug, Derivative)]
#[derivative(Eq, PartialEq, Hash)]
pub struct Repo {
    pub host: String,
    pub name: String,
    pub owner: String,

    #[derivative(PartialEq = "ignore")]
    pub path: Option<PathBuf>,

    #[derivative(PartialEq = "ignore")]
    pub url: String,
}

impl Repo {
    /// # Errors
    ///
    /// Will return `RepoError` if it cannot parse `repo`.
    pub fn from(repo: &str) -> Result<Self, RepoError> {
        let local_repo_path = get_local_repo_path(repo);

        parse_url(
            &get_url(repo, local_repo_path.as_ref())?,
            local_repo_path.as_ref(),
        )
    }

    #[must_use]
    pub fn new(
        host: &str,
        name: &str,
        owner: &str,
        path: Option<PathBuf>,
        url: &str,
    ) -> Self {
        Self {
            host: host.to_string(),
            name: name.to_string(),
            owner: owner.to_string(),
            path,
            url: url.to_string(),
        }
    }

    /// # Errors
    ///
    /// Will return `RepoError` if `Repo` data contains invalid unicode.
    pub fn path(&self, base_directory: &Path) -> Result<String, RepoError> {
        Ok(base_directory
            .join(&self.host)
            .join(&self.owner)
            .join(&self.name)
            .to_str()
            .ok_or(RepoError::RepoPath)?
            .to_owned())
    }
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}/{}", self.host, self.owner, self.name)
    }
}

#[must_use]
pub fn get_local_repo_path(repo: &str) -> Option<PathBuf> {
    let path = tilde(repo).to_string();
    let path = Path::new(&path);

    if Path::is_dir(path) {
        Some(path.to_owned())
    } else {
        None
    }
}

fn get_git_url_from_dir(dir: &Path) -> Result<String, RepoError> {
    Ok(Repository::open(dir)?
        .find_remote("origin")?
        .url()
        .ok_or(RepoError::GitUrl)?
        .to_owned())
}

/// # Errors
///
/// Will return `RepoError` if `local_repo_path` is `Some` but the path doesn't
/// exist or it cannot determine a remote url at that path.
pub fn get_url(
    repo: &str,
    local_repo_path: Option<&PathBuf>,
) -> Result<String, RepoError> {
    local_repo_path.as_ref().map_or_else(
        || Ok(repo.to_string()),
        |path| get_git_url_from_dir(path),
    )
}

/// # Errors
///
/// Will return `RepoError` if it cannot parse `url` or `local_repo_path` is
/// invalid.
pub fn parse_url(
    url: &str,
    local_repo_path: Option<&PathBuf>,
) -> Result<Repo, RepoError> {
    let git_url = GitUrl::parse(url)?;
    let repo_provider = git_url.provider_info::<GenericProvider>()?;

    let url = local_repo_path.as_ref().map_or_else(
        || Ok::<String, RepoError>(url.to_owned()),
        |path| Ok(path.to_str().ok_or(RepoError::GitUrl)?.to_string()),
    )?;

    let local_repo_path = local_repo_path.map(std::borrow::ToOwned::to_owned);

    Ok(Repo::new(
        git_url.host().ok_or(RepoError::GitUrl)?,
        repo_provider.repo(),
        repo_provider.owner(),
        local_repo_path,
        &url,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOST: &str = "github.com";
    const NAME: &str = "src";
    const OWNER: &str = "tymbalodeon";

    fn validate_repo(repo: &Repo, url: &str) {
        assert_eq!(repo.host, HOST);
        assert_eq!(repo.name, NAME);
        assert_eq!(repo.owner, OWNER);
        assert_eq!(repo.url, url);
    }

    #[test]
    fn it_parses_https_url() {
        let url = "https://github.com/tymbalodeon/src.git";
        let repo = parse_url(url, None);

        validate_repo(&repo.unwrap(), url);
    }

    #[test]
    fn it_parses_ssh_url() {
        let url = "git@github.com:tymbalodeon/src.git";
        let repo = parse_url(url, None);

        validate_repo(&repo.unwrap(), url);
    }
}
