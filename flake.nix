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
          binaryen
          cargo-audit
          cargo-edit
          cargo-expand
          cargo-release
          cargo-watch
          nodejs
          (rustChannels.stable.complete.override {
            targets = ["wasm32-unknown-unknown"];
          })
          twiggy
          wabt
          wasm-bindgen-cli
          wasm-pack
        ];
      };
    });
}
