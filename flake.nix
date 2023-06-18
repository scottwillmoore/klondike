{
  inputs = {
    fenix.inputs.nixpkgs.follows = "nix-packages";
    fenix.url = "github:nix-community/fenix";

    flake-utilities.url = "github:numtide/flake-utils";

    nix-packages.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = {
    fenix,
    flake-utilities,
    nix-packages,
    ...
  }:
    flake-utilities.lib.eachDefaultSystem (system: let
      packages = import nix-packages {
        inherit system;
        overlays = [
          fenix.overlays.default
        ];
      };
    in {
      devShells.default = packages.mkShell {
        name = "klondike";
        nativeBuildInputs = [
          packages.fenix.stable.toolchain

          packages.cargo-expand
        ];
      };
    });
}
