{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    openssl
    pkg-config
    lld
  ];
  RUST_BACKTRACE = 1;
}
