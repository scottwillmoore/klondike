{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";

    nix-packages.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    nix-systems.url = "github:nix-systems/default";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nix-packages";
  };
  outputs = inputs @ {
    flake-parts,
    nix-packages,
    nix-systems,
    rust-overlay,
    ...
  }: let
    packagesModule = {
      perSystem = {system, ...}: {
        _module.args.packages = import nix-packages {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
          ];
        };
      };
    };

    systemsModule = {
      systems = import nix-systems;
    };
  in
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        packagesModule
        systemsModule
      ];

      perSystem = {packages, ...}: {
        devShells.default = with packages;
          mkShell {
            name = "klondike";
            nativeBuildInputs = [
              # Node
              nodejs

              # Rust
              cargo-audit
              cargo-edit
              cargo-expand
              cargo-release
              cargo-watch
              (rustChannels.stable.complete.override {
                targets = ["wasm32-unknown-unknown"];
              })

              # WebAssembly
              binaryen
              twiggy
              wabt
              wasm-bindgen-cli
              wasm-pack
            ];
          };
      };
    };
}
