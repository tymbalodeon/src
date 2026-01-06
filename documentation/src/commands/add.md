# Add

Add repositories to the managed folder by passing paths to local repositories, or urls of remote repositories. Remote repositories will be cloned and local repositories moved into the `root_directory` at the path: `<root_directory>/<host>/<owner>/<name>`. If a local path and a remote url represent the same repository, and are both passed simultaneously, the local path will be preferred. If a repository is already managed by `src`, then it will skip it. Use `--force` to override this.

Repositories can be specified as a local path or a remote git url in the form \<host\>:\<owner\>/\<name\>, or omitting the host as \<owner\>/\<name\>, or simply the name, in which case the value for host and/or owner will be pulled from the configuration.
