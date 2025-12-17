pub fn add(repos: &Vec<String>) {
    let repos: Vec<String> =
        repos.iter().map(|repo| format!("- {repo}")).collect();

    println!("Adding repos:\n{}", repos.join("\n"));
}
