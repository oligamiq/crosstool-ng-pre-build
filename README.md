# toolchain for building rustc
A project that provides a toolchain for building the Rust compiler on x86_64 Linux.

*Note: This is a community website and is not officially endorsed by the Rust project.

musl targets are copied from https://musl.cc/

## Usage


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
