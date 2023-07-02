{
  inputs = {
    flake-utilities.url = "github:numtide/flake-utils";

    nix-packages.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    rust-overlay.inputs.flake-utils.follows = "flake-utilities";
    rust-overlay.inputs.nixpkgs.follows = "nix-packages";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = {
    flake-utilities,
    nix-packages,
    rust-overlay,
    ...
  }:
    flake-utilities.lib.eachDefaultSystem (system: let
      packages = import nix-packages {
        inherit system;
        overlays = [
          rust-overlay.overlays.default
        ];
      };
    in {
      devShells.default = packages.mkShell {
        name = "klondike";
        nativeBuildInputs = with packages; [
          cargo-expand
          nodejs
          rustChannels.stable.default
        ];
      };
    });
}
