use std::collections::HashSet;

use anyhow::Result;
use repo::{
    config::{get_root_directory, Config},
    error::SrcRepoError,
    repo::{parse_repos, Repo},
};

use crate::{
    commands::list::{get_host_names, get_owner_names},
    log::{log, LogLevel},
};

type GetValues = fn(bool, bool) -> Result<Vec<String>>;

fn get_values(
    must_exist: bool,
    value: Option<String>,
    getter: GetValues,
) -> Result<Vec<Option<String>>> {
    let mut values: Vec<Option<String>> = vec![];

    if must_exist {
        match value {
            Some(host) => values.push(Some(host)),
            None => values.extend(
                getter(false, false)?
                    .iter()
                    .map(|value| Some(value.to_owned())),
            ),
        }
    } else {
        values.push(value);
    }

    Ok(values)
}

pub fn parse_repos_with_error_log(
    config: &Config,
    repos: &[String],
    host: Option<&String>,
    owner: Option<&String>,
    must_exist: bool,
) -> Result<Vec<Repo>> {
    // TODO: allow repos to be null and select all matching a host and/or owner

    let hosts = get_values(must_exist, host.cloned(), get_host_names)?;
    let owners = get_values(must_exist, owner.cloned(), get_owner_names)?;
    let default_host = config.host.as_deref();
    let default_owner = config.owner.as_deref();

    let mut matching_repos: Vec<Result<Repo, SrcRepoError>> = vec![];

    for host in hosts {
        matching_repos.extend(parse_repos(
            repos,
            default_host,
            default_owner,
            host.as_ref(),
            None,
        ));
    }

    for owner in owners {
        matching_repos.extend(parse_repos(
            repos,
            default_host,
            default_owner,
            owner.as_ref(),
            None,
        ));
    }

    Ok(matching_repos
        .into_iter()
        .filter_map(|repo| match repo {
            Ok(repo) => {
                if must_exist {
                    get_root_directory().map_or(None, |root_directory| {
                        if repo.managed_path(&root_directory).exists() {
                            Some(repo)
                        } else {
                            None
                        }
                    })
                } else {
                    Some(repo)
                }
            }

            Err(error) => {
                log(&LogLevel::Error, &error.to_string());

                None
            }
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect())
}
