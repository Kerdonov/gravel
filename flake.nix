{
  description = "stdsrv Flake file";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utlis.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utlis,
  }:
    flake-utlis.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        stdsrv = import ./package.nix {inherit pkgs;};
      in {
        packages.default = stdsrv;
        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.rustup
            pkgs.bacon
            pkgs.cargo-nextest
            pkgs.cargo-expand
            pkgs.cargo-watch
          ];
        };
      }
    );
}
