use std::fmt;
use std::path::{Path, PathBuf};

use derivative::Derivative;
use git2::Repository;
use git_url_parse::types::provider::GenericProvider;
use git_url_parse::GitUrl;
use shellexpand::tilde;

use crate::error::SrcRepoError;

#[derive(Clone, Debug, Derivative)]
#[derivative(Eq, PartialEq, Hash)]
pub struct Repo {
    pub host: String,
    pub owner: String,
    pub name: String,

    #[derivative(PartialEq = "ignore")]
    pub local_source_path: Option<PathBuf>,

    #[derivative(PartialEq = "ignore")]
    pub url: String,
}

fn parse_url(
    url: &str,
    local_source_path: Option<&PathBuf>,
) -> Result<Repo, SrcRepoError> {
    let git_url = GitUrl::parse(url)?;
    let repo_provider = git_url.provider_info::<GenericProvider>()?;

    let url = local_source_path.as_ref().map_or_else(
        || Ok::<String, SrcRepoError>(url.to_owned()),
        |path| Ok(path.to_str().ok_or(SrcRepoError::GitUrl)?.to_string()),
    )?;

    Ok(Repo {
        host: git_url.host().ok_or(SrcRepoError::GitUrl)?.to_string(),
        owner: repo_provider.owner().clone(),
        name: repo_provider.repo().clone(),
        local_source_path: local_source_path.cloned(),
        url,
        // managed_path: local_source_path.cloned(),
    })
}
impl Repo {
    #[must_use]
    pub fn display(&self, no_host: bool, no_owner: bool) -> String {
        if no_host && no_owner {
            self.name.clone()
        } else if no_host {
            format!("{}/{}", self.owner, self.name)
        } else if no_owner {
            format!("{}:{}", self.host, self.name)
        } else {
            format!("{self}")
        }
    }

    /// # Errors
    ///
    /// Will return `SrcRepoError` if `repo` is a path but the path doesn't
    /// exist or it cannot determine a remote git url at that path.
    pub fn from(repo: &str) -> Result<Self, SrcRepoError> {
        let local_source_path = get_local_source_path(repo);

        let url: Result<String, SrcRepoError> =
            local_source_path.as_ref().map_or_else(
                || Ok(repo.to_string()),
                |path| {
                    Ok(Repository::open(path)?
                        .find_remote("origin")?
                        .url()
                        .ok_or(SrcRepoError::GitUrl)?
                        .to_owned())
                },
            );

        parse_url(&url?, local_source_path.as_ref())
    }

    #[must_use]
    pub fn new(
        host: &str,
        owner: &str,
        name: &str,
        local_source_path: Option<PathBuf>,
        url: &str,
    ) -> Self {
        Self {
            host: host.to_string(),
            name: name.to_string(),
            owner: owner.to_string(),
            local_source_path,
            url: url.to_string(),
        }
    }

    /// # Errors
    ///
    /// Will return `SrcRepoError` if `Repo` data contains invalid unicode.
    pub fn managed_path(
        &self,
        root_directory: &str,
    ) -> Result<String, SrcRepoError> {
        Ok(Path::new(root_directory)
            .join(&self.host)
            .join(&self.owner)
            .join(&self.name)
            .to_str()
            .ok_or(SrcRepoError::RepoPath)?
            .to_owned())
    }
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}/{}", self.host, self.owner, self.name)
    }
}

#[must_use]
pub fn parse_repos(repos: &[String]) -> Vec<Result<Repo, SrcRepoError>> {
    repos.iter().map(|repo| Repo::from(repo)).collect()
}

#[must_use]
pub fn get_local_source_path(repo: &str) -> Option<PathBuf> {
    let path = tilde(repo).to_string();
    let path = Path::new(&path);

    if Path::is_dir(path) {
        Some(path.to_owned())
    } else {
        None
    }
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
