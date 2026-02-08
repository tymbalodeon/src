def get-option [
  options: table<index: int, name: string, value: string>
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
    ^src cd --help

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

  let paths = (
    src list --name $name --path
    | lines
  )

  let paths = if ($paths | is-empty) {
    let paths = (
      src list --path
      | find --no-highlight $name
    )

    if ($paths | is-empty) {
      return
    }

    $paths
  }

  let path = if ($paths | length) > 1 {
    $paths
    | input list
  } else {
    $paths
    | first
  }

  if ($path | path type) == dir {
    cd $path
  }
}
