{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    libiconv
  ];

  RUST_BACKTRACE = 1;
}
