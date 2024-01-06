let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");

  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  unstable = import <nixpkgs-unstable> {};

  # rust = pkgs.rust-bin.stable."1.74.0".default.override {
  #   extensions = [ "rust-src" ];
  # };
  rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
    extensions = [ "rust-src" ];
  });

in
  pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
    gcc
    rust
    # dep
    openssl
    pkg-config
  ];

  shellHook = ''
    echo "Gcc $(gcc --version | head -n 1 | awk '{print $3}'),
    $(rustc --version),
    $(cargo --version),
    $(cargo fmt --version),
    $(cargo clippy --version),
    $(rust-analyzer --version),
    "
  '';

  # Certain Rust tools won't work without this
  # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
  # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
  # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
