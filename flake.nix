{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [cargo2nix.overlays.default];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
	  rustChannel = "nightly";
          packageFun = import ./Cargo.nix;
        };

      in rec {
        packages = {
          rtracer = (rustPkgs.workspace.rtracer {});
          default = packages.rtracer;
        };
      }
    );
}
