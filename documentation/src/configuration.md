# Configuration

To override the default values, configuration can be passed in as optional arguments, read from environment variables, or from a configuration file, with values taking precedence in that order.

To see which optional parameters are available, run `src [COMMAND] --help`.

To pass configuration options as environment variables, prefix the name of the option with `SRC_`. For example, to override the location of the `root_directory`, set the value of `SRC_ROOT_DIRECTORY`.

The default location for the configuration file is `$XDG_CONFIG_HOME/src/config.toml`. To use a configuration file at a different location, pass a path to `--config-file` as an option.

The available configuration options and their default values are shown below.

```toml
host = "github.com"
owner = "<git config github.user>"
root_directory = "$HOME/src"
```
