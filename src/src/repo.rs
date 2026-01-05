use repo::{
    config::{get_root_directory, Config},
    repo::{parse_repos, Repo},
};

use crate::log::{log, LogLevel};

pub fn parse_repos_with_error_log(
    config: &Config,
    repos: &[String],
    must_exist: bool,
) -> Vec<Repo> {
    // TODO: update host
    parse_repos(repos, config.host.as_deref(), config.owner.as_deref())
        .into_iter()
        .filter_map(|repo| match repo {
            Ok(repo) => {
                if must_exist {
                    if repo.managed_path(&get_root_directory().unwrap())
                        .exists()
                    {
                        Some(repo)
                    } else {
                        None
                    }
                } else {
                    Some(repo)
                }
            }

            Err(error) => {
                log(&LogLevel::Error, &error.to_string());

                None
            }
        })
        .collect()
}
