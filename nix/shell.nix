{
  mkShell,
  rust-analyzer,
  rustfmt,
  clippy,
  cargo,
  rustc,
  rustPlatform,
  pkg-config,
  systemd,
}:
mkShell {
  nativeBuildInputs = [
    cargo
    rustc
    rustfmt
    rust-analyzer
    clippy

    pkg-config
    systemd
  ];

  env.RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
}
