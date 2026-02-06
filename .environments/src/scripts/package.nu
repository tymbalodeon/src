#!/usr/bin/env nu

use ../../default/scripts/print.nu print-error

# Manage the nix package
def main [] {
  help main
}

# Build the package
def "main build" [] {
  main generate
  nix build --file Cargo.nix workspaceMembers.src.build
}

# Generate the Cargo.nix file
def "main generate" [] {
  crate2nix generate
}

# Run the built package
def --wrapped "main run" [...args: string] {
  let binary = "./result/bin/src"

  if ($binary | path type) != file {
    main build
  }

  if ($binary | path type) == file {
    run-external $binary ...$args
  } else {
    print-error "package is not built"
  }
}
