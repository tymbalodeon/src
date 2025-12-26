use anyhow::Result;
use repo::list::list_repos;

pub fn list(no_host: bool, no_owner: bool, path: bool) -> Result<()> {
    println!("{}", list_repos(no_host, no_owner, path)?.join("\n"));

    Ok(())
}
