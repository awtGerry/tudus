{
  lib,
  pkgs,
  stdenv,
  rustPlatform,
  installShellFiles,
  darwin,
  version ? "git"
}:

rustPlatform.buildRustPackage rec {
  pname = "tudus";
  inherit version;

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  buildInputs = with pkgs; [
    pkg-config
    gtk-layer-shell
    gtk3
    cmake

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
  ]
  ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.SystemConfiguration
    darwin.apple_sdk.frameworks.Foundation
  ];

  nativeBuildInputs = with pkgs; [
    installShellFiles
    pkg-config
    gtk-layer-shell
    gtk3
    cmake
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

  postInstall = ''
    installShellCompletion --cmd tudus \
      --bash <($out/bin/tudus completions bash) \
      --zsh <($out/bin/tudus completions zsh) \
      --fish <($out/bin/tudus completions fish)
  '';

  patchPhase = ''
    sed -i 's/env!("CARGO_PKG_VERSION")/\"${version}\"/g' src/main.rs
  '';

  meta = with lib;{
    description = "Basic todo gui app";
    homepage = "https://github.com/awtgerry/tudus";
    license = licenses.lgpl3Only;
    mainProgram = "tudus";
  };
}
