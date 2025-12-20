use repo::parse_repo;
use repo::Repo;

use crate::config::get_config;

pub fn add(repos: &[String]) {
    let mut repos_to_add: Vec<Repo> = repos
        .iter()
        .filter_map(|repo| {
            let parsed_repo = parse_repo(repo).unwrap();

            if parsed_repo.path.is_some() {
                Some(parsed_repo)
            } else {
                None
            }
        })
        .collect();

    let remote_repos: Vec<Repo> = repos
        .iter()
        .filter_map(|repo| {
            let parsed_repo = parse_repo(repo).unwrap();

            if parsed_repo.path.is_none()
                && !repos_to_add.contains(&parsed_repo)
            {
                Some(parsed_repo)
            } else {
                None
            }
        })
        .collect();

    repos_to_add.extend(remote_repos);

    let root_directory = get_config().root_directory;

    for repo in repos_to_add {
        println!("Adding {} to {}", repo.url, repo.path(&root_directory));
    }
}
