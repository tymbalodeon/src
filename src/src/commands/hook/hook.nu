def --env --wrapped "src cd" [...args: string] {
  let repo = (
    $args
    | where {not ($in | str starts-with "--")}
    | first
  )

  cd (just run list --name $repo --path | str trim)
}
