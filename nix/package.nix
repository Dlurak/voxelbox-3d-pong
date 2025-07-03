{ pkgs }:
let
  manifest = pkgs.lib.importTOML ../Cargo.toml;
in
pkgs.rustPlatform.buildRustPackage {
  pname = manifest.package.name;
  version = manifest.package.version;

  src = pkgs.lib.cleanSource ./..;
  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];
  buildInputs = with pkgs; [ libudev-zero ];

  meta = with pkgs; {
    description = "The classic game pong for the 3d voxelbox";
    homepage = "https://github.com/voxelbox-3d-pong/";
    mainProgram = manifest.package.name;
    maintainers = [ lib.maintainers.dlurak ];
  };
}
