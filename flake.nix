{
  description = "bm-darwin-setup — Darwin system activation tool for blackmatter";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    crate2nix.url = "github:nix-community/crate2nix";
    flake-utils.url = "github:numtide/flake-utils";
    substrate = {
      url = "github:pleme-io/substrate";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crate2nix,
      flake-utils,
      substrate,
    }:
    (import "${substrate}/lib/rust-tool-release-flake.nix" {
      inherit nixpkgs crate2nix flake-utils;
    })
      {
        toolName = "bm-darwin-setup";
        src = self;
        repo = "pleme-io/bm-darwin-setup";
      };
}
