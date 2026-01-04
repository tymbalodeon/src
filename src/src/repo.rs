use repo::repo::{parse_repos, Repo};

use crate::log::{log, LogLevel};

pub fn parse_repos_with_error_log(repos: &[String]) -> Vec<Repo> {
    parse_repos(repos)
        .into_iter()
        .filter_map(|repo| match repo {
            Ok(repo) => Some(repo),

            Err(error) => {
                log(&LogLevel::Error, &error.to_string());

                None
            }
        })
        .collect()
}
