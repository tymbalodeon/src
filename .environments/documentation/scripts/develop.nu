#!/usr/bin/env nu

# Open a pre-configured development environment
def main [] {
  zellij --layout $"($env.ENVIRONMENTS)/documentation/layout.kdl"
}
