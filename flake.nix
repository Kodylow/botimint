{
  description = "Botimint: Fedimint Discord Bot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";

    flakebox = {
      url = "github:rustshop/flakebox";
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
        legacyPackages = outputs // { clightning = pkgs.clightning; };
        devShells = flakeboxLib.mkShells {
          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.starship
            pkgs.clightning
            pkgs.just
          ];
          packages = [ ];
          shellHook = ''
              eval "$(starship init bash)"
          '';
        };
      });
}
