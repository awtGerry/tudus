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

  # nativeBuildInputs = oldAttrs.nativeBuildInputs or [] ++ [ pkgs.makeWrapper ];
  nativeBuildInputs = with pkgs; [
    installShellFiles
    pkg-config
    gtk-layer-shell
    gtk3
    cmake
    makeWrapper
    wrapGAppsHook
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

  postFixup = ''
    wrapProgram $out/bin/tudus \
      --suffix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath buildInputs}
  '';

  meta = with lib;{
    description = "Basic todo gui app";
    homepage = "https://github.com/awtgerry/tudus";
    license = licenses.lgpl3Only;
    mainProgram = "tudus";
  };
}
