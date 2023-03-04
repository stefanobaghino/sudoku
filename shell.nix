let
  pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/22.11.tar.gz")) {};
in pkgs.mkShell {
  buildInputs = [ pkgs.cargo pkgs.rustc pkgs.libiconv ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  RUST_BACKTRACE = 1;
}
