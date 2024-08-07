{
  description = "iced";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };

      buildInputs = with pkgs; [
        vulkan-loader
        # for wayland
        wayland
        wayland-protocols

        # for x11
        xorg.libX11
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi

        libxkbcommon
      ];
      my-crate = crane.lib.${system}.buildPackage {
        src = ./.;
        inherit buildInputs;

        nativeBuildInputs = with pkgs; [
          pkg-config
          gtk-layer-shell
          gtk3
          cmake
        ];
      };
    in {
      checks = {
        inherit my-crate;
      };

      packages.default = my-crate;

      apps.default = flake-utils.lib.mkApp {
        drv = my-crate;
      };

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks;

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        # Extra inputs can be added here
      };
    });
}
