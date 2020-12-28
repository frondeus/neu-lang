let mozillaOverlay = import ( 
  builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz 
  );
  nixpkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
  rust = ( nixpkgs.rustChannelOf { rustToolchain = ./rust-toolchain; }).rust.override { extensions = ["rust-src" "rustfmt-preview" "clippy-preview"]; };

in
  with nixpkgs; pkgs.mkShell {
    buildInputs = [
      clang
      cmake
      pkg-config
      rust
      colordiff
    ];

    LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
  }
