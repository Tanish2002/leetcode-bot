{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self
    , nixpkgs
    , fenix
    , flake-utils
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rustToolchain = fenix.packages.${system}.stable.withComponents [
          "rustc"
          "cargo"
          "rustfmt"
          "clippy"
          "rust-analyzer"
          "rust-src"
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          name = "leetcode-bot";

          buildInputs = with pkgs; [
            # Rust toolchain from fenix
            rustToolchain

            # AWS related tools
            awscli2
            cargo-lambda

            # Terraform/OpenTofu
            opentofu
            terraform-ls

            # Debugging tools
            jq
            curl
          ];
        };
      }
    );
}
