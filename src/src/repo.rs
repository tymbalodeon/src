use repo::repo::{parse_repos, Repo};

use crate::log::{log, LogLevel};

pub fn get_repos(repos: &[String]) -> Vec<Repo> {
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
