{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    # nixpkgs 21808d22b1cda1898b71cf1a1beb524a97add2c4
    #   https://lazamar.co.uk/nix-versions/?channel=nixos-unstable&package=cargo
    #   cargo 1.83, rust-analyzer 2025-01-08
    nixpkgs.url = "github:nixos/nixpkgs/21808d22b1cda1898b71cf1a1beb524a97add2c4";
  };

  outputs = { flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.cargo
            pkgs.rust-analyzer
          ];
        };
      }
    );
}
