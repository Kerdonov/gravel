{pkgs}:
pkgs.rustPlatform.buildRustPackage {
  pname = "stdsrv";
  version = "0.1.0";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  cargoBuildFlags = ["-p" "stdsrv"];

  doCheck = true;

  meta = with pkgs.lib; {
    description = "A simple file server that converts your markdown files to HTML before serving them.";
    license = licenses.gpl3;
    maintainers = [maintainers.scrac];
    platforms = platforms.linux;
  };
}
