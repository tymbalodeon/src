#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn no_args_is_help() {
        let mut command = Command::cargo_bin("src").unwrap();

        let help_text = "\
Manage git repositories in an organized way

Usage: src [COMMAND]

Commands:
  cd      Change directory
  clone   Clone
  config  Config
  ls      List
  new     New
  rm      Remove
  sync    Sync
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
";

        command.assert().stderr(help_text);
    }
}
