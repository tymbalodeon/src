use std::path::{Path, PathBuf};

use derivative::Derivative;
use git2::Repository;
use git_url_parse::types::provider::GenericProvider;
use git_url_parse::{GitUrl, GitUrlParseError};
use shellexpand::tilde;

#[derive(Debug, Derivative)]
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
    #[must_use]
    pub fn from(
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

    #[must_use]
    pub fn path(&self, base_directory: &PathBuf) -> String {
        base_directory
            .join(&self.host)
            .join(&self.owner)
            .join(&self.name)
            .to_str()
            .unwrap()
            .to_owned()
    }
}

fn get_local_repo_path(repo: &str) -> Option<PathBuf> {
    let path = tilde(repo).to_string();
    let path = Path::new(&path);

    if Path::is_dir(path) {
        Some(path.to_owned())
    } else {
        None
    }
}

fn get_git_url_from_dir(dir: &Path) -> String {
    Repository::open(dir)
        .unwrap()
        .find_remote("origin")
        .unwrap()
        .url()
        .unwrap()
        .to_owned()
}

fn get_url(repo: &str, local_repo_path: &Option<PathBuf>) -> String {
    match local_repo_path {
        Some(path) => get_git_url_from_dir(&path),
        None => repo.to_string(),
    }
}

fn parse_url(
    url: &str,
    local_repo_path: &Option<PathBuf>,
) -> Result<Repo, GitUrlParseError> {
    let git_url = GitUrl::parse(url).unwrap();
    let repo_provider = git_url.provider_info::<GenericProvider>().unwrap();

    let url = match local_repo_path {
        Some(path) => path.to_str().unwrap().to_string(),
        None => url.to_owned(),
    };

    Ok(Repo::from(
        git_url.host().unwrap(),
        repo_provider.repo(),
        repo_provider.owner(),
        local_repo_path.to_owned(),
        &url,
    ))
}

pub fn parse_repo(repo: &str) -> Result<Repo, GitUrlParseError> {
    let local_repo_path = get_local_repo_path(repo);

    parse_url(&get_url(repo, &local_repo_path), &local_repo_path)
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
        let repo = parse_url(url, &None).unwrap();

        validate_repo(&repo, url);
    }

    #[test]
    fn it_parses_ssh_url() {
        let url = "git@github.com:tymbalodeon/src.git";
        let repo = parse_url(url, &None).unwrap();

        validate_repo(&repo, url);
    }
}
