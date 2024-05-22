{
  description = "Simple todo gui app";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    crane,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        craneLib = crane.lib.${system};

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        libPath = with pkgs; lib.makeLibraryPath [
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

        nativeBuildInputs = with pkgs; [
          pkg-config
          gtk-layer-shell
          gtk3
          cmake
        ];

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

        cargoArtifacts = craneLib.buildDepsOnly ({
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          inherit buildInputs nativeBuildInputs;
          pname = "tudus";
        });
      in with pkgs; {
        packages = rec {
          tudus = craneLib.buildPackage {
            src = craneLib.path ./.;

            inherit buildInputs nativeBuildInputs cargoArtifacts;

            postInstall = ''
              install -Dm644 "$src/app/tudus.desktop" -t "$out/share/applications/"

              patchelf --set-rpath ${libPath} $out/bin/tudus

              wrapProgram $out/bin/tudus \
                --prefix PATH : ${lib.makeBinPath [ pkgs.gnome.zenity pkgs.libsForQt5.kdialog]}\
            '';

            GIT_HASH = self.rev or self.dirtyRev;
          };

          default = tudus;
        };

        devShell = mkShell {
          inherit buildInputs nativeBuildInputs;

          packages = with pkgs; [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
            })
            cargo-watch
            gnome.zenity
            libsForQt5.kdialog
          ];
          LD_LIBRARY_PATH = "${libPath}";
        };
      }) // {
        overlay = final: prev: {
          inherit (self.packages.${final.system}) tudus;
        };
      };
}
