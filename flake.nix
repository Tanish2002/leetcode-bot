{
  description = "LeetCode-Bot DevShell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.05";
    devenv.url = "github:cachix/devenv";
  };
  outputs = {
    self,
    nixpkgs,
    devenv,
    ...
  } @ inputs: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
  in {
    devShell.x86_64-linux = devenv.lib.mkShell {
      inherit inputs pkgs;
      modules = [
        (_: {
          languages.go.enable = true;
          languages.terraform.enable = true;
          packages = [pkgs.awscli2];
        })
      ];
    };
  };
}
