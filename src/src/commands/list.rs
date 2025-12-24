use repo::list::list as list_repos;

pub fn list(as_path: bool) {
    println!("{}", list_repos(as_path).join("\n"));
}
