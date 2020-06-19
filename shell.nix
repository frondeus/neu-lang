{pkgs ? import <nixpkgs> {}, lib ? pkgs.stdenv.lib }:
pkgs.stdenv.mkDerivation rec {
    name = "neu";
    nativeBuildInputs = with pkgs; [ pkgconfig ];
    buildInputs = with pkgs; [
        gcc
        git
        colordiff
        openssl.dev

        # Trying Iced GUI
        xorg.libX11
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
        freetype
        expat
        gperf
        python3
        vulkan-headers
        vulkan-loader
        vulkan-tools
    ];
  OPENSSL_DEV = pkgs.openssl.dev;
  LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
}

