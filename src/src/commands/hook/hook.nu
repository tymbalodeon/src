def get-option [
  options: table<value: int, name: string, index: int>
  option: string
] {
  let value = ($options | where name == $option)

  if ($value | is-not-empty) {
    $value
    | first
    | get value
  }
}

def --env --wrapped "src cd" [...args: string] {
  if "-h" in $args or "--help" in $args {
    # FIXME: change to `src cd` when ready!
    just run cd --help

    return
  }

  mut option_indices = []

  let options = (
    $args
    | enumerate
    | where {$in.item | str starts-with "--"}
    | each {
        |option|

        {
          value: ($args | get ($option.index + 1))
          name: ($option.item | str replace "--" "")
          index: $option.index
        }
      }
    | where name in [host name owner]
  )

  let host = (get-option $options host)
  let name = (get-option $options name)
  let owner = (get-option $options owner)

  let name = if ($name | is-empty) {
    let name = (
      $args
      | enumerate
      | where $it.index not-in (
          $options.index
          | append ($options.index | each {$in + 1})
        )
    )

    if ($name | is-not-empty) {
      $name
      | first
      | get item
    }
  }

  if ($name | is-empty) {
    return
  }

  # FIXME: change to `src list` when ready!
  let path = (
    just run list --name $name --path
    | lines
  )

  if ($path | is-empty) {
    return
  }

  let path = if ($path | length) > 1 {
    $path
    | input list
  } else {
    $path
    | first
  }

  if ($path | path type) == dir {
    cd $path
  }
}
