# Add

Add repositories to the managed folder by passing paths to local repositories or git urls of remote repositories. Remote repositories will be cloned and local repositories moved (or copied with `--copy`) into the `root_directory` at the path: `<root_directory>/<host>/<owner>/<name>`. If a local path and a remote url represent the same repository, and are both passed simultaneously, the local path will be preferred, in order to preserve any local work that has not yet been pushed to the remote. If a repository is already managed by `src`, then it will skip it. Use `--force` to override this.

Git urls can be in the form `<host>:\owner>/<name>`, `<owner>/<name>`, or, simply, `<name>`. Any components that are missing will be filled in with values from the [coniguration](../configuration.md).
