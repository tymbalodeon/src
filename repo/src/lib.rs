use std::path::Path;

use git2::Repository;
use git_url_parse::types::provider::GenericProvider;
use git_url_parse::{GitUrl, GitUrlParseError};
use shellexpand::tilde;

fn get_git_url_from_dir(dir: &Path) -> String {
    Repository::open(dir)
        .unwrap()
        .find_remote("origin")
        .unwrap()
        .url()
        .unwrap()
        .to_owned()
}

#[derive(Debug, Eq, Hash)]
pub struct Repo {
    pub host: String,
    pub name: String,
    pub owner: String,
}

impl Repo {
    fn from(host: &str, name: &str, owner: &str) -> Self {
        Repo {
            host: host.to_string(),
            name: name.to_string(),
            owner: owner.to_string(),
        }
    }
}

impl PartialEq for Repo {
    fn eq(&self, other: &Self) -> bool {
        self.host == other.host
            && self.name == other.name
            && self.owner == other.owner
    }
}

fn parse_url(url: &str) -> Result<Repo, GitUrlParseError> {
    let url = GitUrl::parse(url).unwrap();
    let repo_provider = url.provider_info::<GenericProvider>().unwrap();

    Ok(Repo::from(
        url.host().unwrap(),
        repo_provider.repo(),
        repo_provider.owner(),
    ))
}

fn get_url(repo: &str) -> String {
    let path = tilde(repo).to_string();
    let path = Path::new(&path);

    if Path::is_dir(path) {
        get_git_url_from_dir(path)
    } else {
        repo.to_string()
    }
}

pub fn parse_repo(repo: &str) -> Result<Repo, GitUrlParseError> {
    parse_url(&get_url(repo))
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOST: &str = "github.com";
    const NAME: &str = "src";
    const OWNER: &str = "tymbalodeon";

    fn validate_repo(repo: &Repo) {
        assert_eq!(repo.host, HOST);
        assert_eq!(repo.name, NAME);
        assert_eq!(repo.owner, OWNER);
    }

    #[test]
    fn it_parses_https_url() {
        let repo =
            parse_url("https://github.com/tymbalodeon/src.git").unwrap();

        validate_repo(&repo);
    }

    #[test]
    fn it_parses_ssh_url() {
        let repo = parse_url("git@github.com:tymbalodeon/src.git").unwrap();

        validate_repo(&repo);
    }
}
