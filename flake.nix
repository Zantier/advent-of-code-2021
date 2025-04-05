{
  inputs = {
    # nixpkgs 21808d22b1cda1898b71cf1a1beb524a97add2c4
    #   https://lazamar.co.uk/nix-versions/?channel=nixos-unstable&package=cargo
    #   cargo 1.83, rust-analyzer 2025-01-08
    nixpkgs.url = "github:nixos/nixpkgs/21808d22b1cda1898b71cf1a1beb524a97add2c4";
  };

  outputs = { nixpkgs, ... }:
    let
      systems = [ "aarch64-darwin" "aarch64-linux" "x86_64-darwin" "x86_64-linux" ];
      forEachSystem = function:
        nixpkgs.lib.genAttrs systems (system: function nixpkgs.legacyPackages.${system});
    in {
      devShells = forEachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = [
            pkgs.cargo
            pkgs.rust-analyzer
          ];
        };
      });
    };
}
