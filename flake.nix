{
  description = "gravel project flake";

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
        gravel = import ./package.nix {inherit pkgs;};
      in {
        packages.default = gravel;
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustup
            helix
            bacon
            cargo-nextest
            cargo-expand
            cargo-watch
            cargo-tarpaulin
          ];
        };
      }
    );
}
