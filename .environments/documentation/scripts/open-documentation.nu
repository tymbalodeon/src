#!/usr/bin/env nu

# Open the documentation page
export def main [] {
  job spawn { start http://localhost:3000/ }
}
