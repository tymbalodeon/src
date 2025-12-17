#!/usr/bin/env nu

use open-documentation.nu

# Serve documentation files and recompile on changes
def main [] {
  open-documentation
  mdbook serve documentation
}
