let
  pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/8021ed2090498ff171ad339c2b2eac73d4755a13.tar.gz")) {};
in
pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.rustc
    pkgs.rustfmt
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
 
