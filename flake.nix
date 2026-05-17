{
  inputs = {
    crate2nix = {
      inputs.nixpkgs.follows = "nixpkgs";
      url = "github:nix-community/crate2nix";
    };

    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    nutest = {
      flake = false;
      url = "github:vyadh/nutest";
    };

    systems.url = "github:nix-systems/default";
  };

  outputs = {
    crate2nix,
    nixpkgs,
    systems,
    ...
  }: {
    packages = nixpkgs.lib.genAttrs (import systems) (system: {
      default = let
        cargoNix = crate2nix.tools.${system}.appliedCargoNix {
          name = "src";
          src = ./.;
        };
      in
        cargoNix.workspaceMembers.src.build;
    });
  };
}
