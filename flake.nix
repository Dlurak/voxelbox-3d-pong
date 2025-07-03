{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      forAllSystems =
        function:
        nixpkgs.lib.genAttrs [ "aarch64-linux" "x86_64-linux" ] (
          system: function nixpkgs.legacyPackages.${system}
        );
    in
    {
      formatter = forAllSystems (pkgs: pkgs.nixfmt-tree);
      packages = forAllSystems (
        pkgs:
        let
        in
        rec {
          pong = import ./nix/package.nix { inherit pkgs; };
          default = pong;
        }
      );
      devShells = forAllSystems (
        pkgs: with pkgs; {
          default = mkShell {
            buildInputs = [
              cargo
              rustc
              rustfmt
              pre-commit
              rustPackages.clippy
              rust-analyzer
              bacon
              libudev-zero
              pkg-config
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
        }
      );
    };
}
