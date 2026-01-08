# TODO: finish me!

def --env --wrapped "src cd" [...rest: string] {
  let repo = (
    $rest
    | where {"--" not-in $in}
    | first
  )

  cd (just run list --name $repo --path | str trim)
}

