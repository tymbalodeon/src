use anyhow::Result;
use repo::list::list as list_repos;

pub fn list(as_path: bool) -> Result<()> {
    println!("{}", list_repos(as_path)?.join("\n"));

    Ok(())
}
