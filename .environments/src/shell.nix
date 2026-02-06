{pkgs, ...}: {
  packages = with pkgs; [
    crate2nix
    dioxus-cli
  ];
}
