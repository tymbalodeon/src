use anyhow::Result;
use repo::repo::parse_repos;

pub fn remove(repos: &[String]) -> Result<()> {
    let repos = parse_repos(repos);

    dbg!(repos);
    println!("Removing repos");

    Ok(())
}
