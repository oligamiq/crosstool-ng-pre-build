# toolchain for building rustc
A project that provides a toolchain for building the Rust compiler on x86_64 Linux.

*Note: This is a community website and is not officially endorsed by the Rust project.

musl targets are copied from https://musl.cc/

## Usage
```bash
cargo install --git https://github.com/oligamiq/toolchain-for-building-rustc
toolchain-for-building-rustc --tier 1 --tier 2-host --os linux -t sparcv9-sun-solaris -t arm-unknown-linux-musleabi -t arm-unknown-linux-musleabihf -t i586-unknown-linux-gnu -t i686-unknown-linux-musl
```
Currently, only these are supported. For more details, please refer to: `https://github.com/oligamiq/rust_wasm/blob/82e56231caf29dbba4c28a719bfef1ac89772234/.github/workflows/rustc_llvm_with_lld.yml`.

### about install
All file is installed in `/x-tools` directory.
Place the symbolic link for bin in /usr/local/bin.

## !! Caution !!
Several musl targets are not tested yet and it depends on the gnu toolchain.

## compile check
Compile check is done, but it is not guaranteed to work.
Compile check on c, c++.

## TODO
- [] Check downloaded files by signature.
- [] Refactor the error handling.
- [] Handle ctrl-c and safely exit.

## Related Projects
- https://musl.cc/
