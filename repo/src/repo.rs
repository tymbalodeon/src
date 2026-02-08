use std::fmt;
use std::fmt::Write;
use std::path::{Path, PathBuf};

use derivative::Derivative;
use git_url_parse::{GitUrl, types::provider::GenericProvider};
use git2::Repository;
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
    url: String,
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

        let url = local_source_path.as_ref().map_or_else(
            || Ok::<String, SrcRepoError>(repo.to_string()),
            |path| {
                Ok(Repository::open(path)?
                    .find_remote("origin")?
                    .url()
                    .ok_or(SrcRepoError::GitUrl)?
                    .to_owned())
            },
        )?;

        parse_url(&url, local_source_path.as_ref())
    }

    #[must_use]
    pub fn managed_path(&self, root_directory: &str) -> PathBuf {
        PathBuf::from(root_directory)
            .join(&self.host)
            .join(&self.owner)
            .join(&self.name)
    }

    #[must_use]
    pub fn managed_path_name(&self, root_directory: &str) -> String {
        self.managed_path(root_directory)
            .to_string_lossy()
            .to_string()
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

    #[must_use]
    pub fn url(self) -> String {
        format!("git@{}", self.url)
    }
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}/{}", self.host, self.owner, self.name)
    }
}

#[must_use]
pub fn parse_repos(
    repos: &[String],
    default_host: Option<&str>,
    default_owner: Option<&str>,
    host_filter: Option<&String>,
    owner_filter: Option<&String>,
) -> Vec<Result<Repo, SrcRepoError>> {
    repos
        .iter()
        .map(|repo| {
            Repo::from(repo).map_or_else(
                |_| {
                    let mut owner: Option<&str> = None;
                    let name: Option<&str>;
                    let owner_separator = '/';

                    if repo.contains(owner_separator) {
                        let mut components = repo.split(owner_separator);

                        owner = components.next();
                        name = components.next();
                    } else {
                        name = repo.split('/').next_back();
                    }

                    match owner {
                        Some(owner) => {
                            if let Some(owner_filter) = owner_filter
                                && owner.to_lowercase()
                                    != owner_filter.to_lowercase()
                            {
                                return Err(SrcRepoError::Config);
                            }
                        }

                        None => {
                            if let Some(owner_filter) = owner_filter {
                                owner = Some(owner_filter);
                            } else {
                                owner = default_owner;
                            }
                        }
                    }

                    let mut host: Option<&str> = None;
                    let host_separator = ':';

                    if repo.contains(host_separator) {
                        let mut components = repo.split(host_separator);

                        host = components.next();
                    }

                    match host {
                        Some(host) => {
                            if let Some(host_filter) = host_filter
                                && host.to_lowercase()
                                    != host_filter.to_lowercase()
                            {
                                return Err(SrcRepoError::Config);
                            }
                        }

                        None => {
                            if let Some(host_filter) = host_filter {
                                host = Some(host_filter);
                            } else {
                                host = default_host;
                            }
                        }
                    }

                    let mut url: String = String::new();

                    if let Some(host) = host {
                        let _ = write!(url, "{host}:");
                    }

                    if let Some(owner) = owner {
                        let _ = write!(url, "{owner}");
                    }

                    if let Some(name) = name {
                        let _ = write!(url, "{}", &format!("/{name}"));
                    }

                    Repo::from(&url)
                },
                Ok,
            )
        })
        .filter_map(|repo| match repo {
            Ok(repo) => Some(Ok(repo)),
            Err(SrcRepoError::Filter) => None,
            _ => Some(repo),
        })
        .collect()
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
