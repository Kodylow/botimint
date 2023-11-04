{
  description = "Botimint: Fedimint Discord Bot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";

    flakebox = {
      url = "github:rustshop/flakebox?rev=9e45d2c0b330a170721ada3fe3a73c38dcff763b";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flakebox, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        flakeboxLib = flakebox.lib.${system} { };

        rustSrc = flakeboxLib.filterSubPaths {
          root = builtins.path {
            name = "botimint";
            path = ./.;
          };
          paths = [
            "Cargo.toml"
            "Cargo.lock"
            ".cargo"
            "src"
          ];
        };

        outputs =
          (flakeboxLib.craneMultiBuild { }) (craneLib':
            let
              craneLib = (craneLib'.overrideArgs {
                pname = "flexbox-multibuild";
                src = rustSrc;
                nativeBuildInputs = [
                  pkgs.pkg-config
                ];
              });
            in
            rec {
              workspaceDeps = craneLib.buildWorkspaceDepsOnly { };
              workspaceBuild = craneLib.buildWorkspace {
                cargoArtifacts = workspaceDeps;
              };
              flakebox-tutorial = craneLib.buildPackage { };
            });
      in
      {
        legacyPackages = outputs;
        devShells = flakeboxLib.mkShells {
          nativeBuildInputs = [
            pkgs.pkg-config
          ];
          packages = [ ];
        };
      });
}
