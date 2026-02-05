def --env --wrapped "src cd" [...args: string] {
  if "-h" in $args or "--help" in $args {
    # FIXME: change to `src cd` when ready!
    just run cd --help

    return
  }

  let repo = (
    $args
    | where {not ($in | str starts-with "--")}
    | first
  )

  if ($repo | is-empty) {
    return
  }

  let path = (
    # FIXME: change to `src list` when ready!
    just run list --name $repo --path
    | str trim
  )

  if ($path | is-not-empty) {
    cd $path
  }
}
