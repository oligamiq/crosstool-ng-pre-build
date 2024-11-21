pub fn musl_require_root() -> [&'static str; 6] {
  [
    "armv5te-unknown-linux-musleabi",
    "armv7-unknown-linux-musleabi",
    "armv7-unknown-linux-musleabihf",
    "armv7hf-unknown-linux-musleabihf",
    "i586-unknown-linux-musl",
    "riscv64gc-unknown-linux-musl",
  ]
}
