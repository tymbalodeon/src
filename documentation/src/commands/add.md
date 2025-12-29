# Add

Add repositories to the managed folder by passing paths to local repositories, or urls of remote repositories. Remote repositories will be cloned and local repositories moved into the `root_directory` at the path: `<root_directory>/<host>/<owner>/<name>`. If a local path and a remote url represent the same repository, and are both passed simultaneously, the local path will be preferred. If a repository is already managed by `src`, then it will skip it. Use `--force` to override this.
