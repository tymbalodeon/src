use std::collections::HashSet;

use repo::parse_repo;
use repo::Repo;

pub fn add(repos: &Vec<String>) {
    let repos: HashSet<Repo> =
        repos.iter().map(|repo| parse_repo(repo).unwrap()).collect();

    println!("{repos:#?}");
}
