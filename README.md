# Crosstool-NG Pre-built Toolchains
Pre-built GCC toolchains using Crosstool-NG.
*Note: This is a community website and is not officially endorsed by Crosstool-NG.*

This is by rust toolchain, and it is used to build the rust toolchain.

musl targets are copied from https://musl.cc/

## Usage
Target is on rust toolchain.
https://doc.rust-lang.org/nightly/rustc/platform-support.html

### Musl Targets
On building rustc with musl target, it require musl-root.
*Note: musl-root is including libc.a.

| **target**                | **description**           |**musl-root**             |**on rust**             |
|---------------------------|---------------------------|--------------------------|--------------------------|
| `aarch64-unknown-linux-musl` | for aarch64-linux-musl | `aarch64-unknown-linux-musl/aarch64-linux-musl/lib` |
| `riscv64-unknown-linux-musl` | for riscv64-linux-musl | `riscv64-unknown-linux-musl/riscv64-linux-musl/lib` | false |
| `loongarch64-unknown-linux-musl`

## install
Provides a pre-built GCC toolchain that runs on x86_64 Linux systems for cross-compilation to various targets.
If you need a toolchain for architectures other than x86_64, please open an issue.

## compile check
Compile check is done, but it is not guaranteed to work.

## Related Projects
- https://musl.cc/
