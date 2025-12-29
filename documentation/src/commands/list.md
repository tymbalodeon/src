# List

By default, `list` displays `src`-managed repositories. To show non-`src`-managed repositories on your system, use `--non-managed`, or `--all` to display both managed and non-managed repositories.

By default, `list` displays repositories in the form \<host\>:\<owner\>/<\name\>. To display the path, use `--path`. You can control which data is displayed by using `--no-host`, `--no-owner`, or by running `list hosts`, `list owners`, or `list names` (the same as `--no-host --no-owner`).

Default settings for `list` can be configured in the configuration file. See [configuration](../configuration.md).
