{ lib
, fetchFromGitHub
, rustPlatform
, pkgs
}:

rustPlatform.buildRustPackage rec {
  pname = "tudus";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "awtgerry";
    repo = "tudus";
    rev = "v${version}";
    sha256 = "";
  };

  cargoSha256 = "";

  nativeBuildInputs = with pkgs; [
    pkg-config
    cmake
    makeWrapper
  ];

  buildInputs = with pkgs; [
    fontconfig

    vulkan-headers
    vulkan-loader
    libGL

    libxkbcommon

    # wayland
    wayland

    # x11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libX11
  ];

  postFixup = ''
    wrapProgram $out/bin/ytdlp-gui \
      --suffix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath buildInputs}
  '';

  meta = with lib; {
    description = "Basic todo gui app";
    homepage = "https://github.com/awtgerry/tudus/";
    license = licenses.agpl3Only;
    # maintainers = with maintainers; [ awtgerry ];
  };
}
