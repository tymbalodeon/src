use repo::list::list as list_repos;

pub fn list() {
    println!("{}", list_repos().join("\n"));
}
