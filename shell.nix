with import <nixpkgs> {};

mkShell {
  nativeBuildInputs = [
    rustc cargo libiconv
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}
