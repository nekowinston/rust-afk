{
  description = "rust-afk";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {
        self',
        pkgs,
        ...
      }: {
        devShells.default = pkgs.mkShell {
          inputsFrom = [self'.packages.rust-afk];
          buildInputs = [pkgs.rust-analyzer pkgs.clippy pkgs.rustfmt];
          env.RUSTC_SRC_PATH = "${pkgs.rustc}/lib/rustlib/src/rust/src";
        };
        packages = let
          src = pkgs.lib.cleanSource ./.;
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          yarnProject = pkgs.callPackage ./nix/yarn-project.nix {} {inherit src;};
        in rec {
          default = rust-afk;
          rust-afk = pkgs.rustPlatform.buildRustPackage {
            inherit (cargoToml.package) name version;
            inherit src;

            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = yarnProject.buildInputs;
            preBuild = ''
              ${yarnProject.configurePhase}
              yarn build
            '';
          };
        };
      };
    };
}
