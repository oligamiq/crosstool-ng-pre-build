#!/bin/bash
# ignore-tidy-linelength

set -eux

arch=$1
binutils_version=2.40
freebsd_version=12.3
triple=$arch-unknown-freebsd12
prefix="/x-tools/$triple"

sysroot="$prefix/sysroot"

hide_output() {
  set +x
  local on_err="
echo ERROR: An error was encountered with the build.
cat /tmp/build.log
exit 1
"
  trap "$on_err" ERR
  bash -c "while true; do sleep 30; echo \$(date) - building ...; done" &
  local ping_loop_pid=$!
  "$@" &> /tmp/build.log
  trap - ERR
  kill $ping_loop_pid
  set -x
}

# Next, download the FreeBSD libraries and header files
mkdir -p "$sysroot"
case $arch in
  (x86_64) freebsd_arch=amd64 ;;
  (i686) freebsd_arch=i386 ;;
esac

sudo chown -R "$(id -u):$(id -g)" $sysroot
sudo chmod -R u+w $sysroot

# Originally downloaded from:
# URL=https://download.freebsd.org/ftp/releases/${freebsd_arch}/${freebsd_version}-RELEASE/base.txz
URL=https://ci-mirrors.rust-lang.org/rustc/2022-05-06-freebsd-${freebsd_version}-${freebsd_arch}-base.txz
curl "$URL" | tar xJf - -C "$sysroot" ./usr/include ./usr/lib ./lib

# First up, build binutils
mkdir binutils
cd binutils
curl -L https://ftp.gnu.org/gnu/binutils/binutils-${binutils_version}.tar.bz2 | tar xjf -
mkdir binutils-build
cd binutils-build
hide_output ../binutils-${binutils_version}/configure \
  --target="$triple" --with-sysroot="$sysroot" --prefix="$prefix"
hide_output make -j`nproc`
hide_output make install
cd ../..
rm -rf binutils

export PATH="$prefix/bin:$PATH"

# Clang can do cross-builds out of the box, if we give it the right
# flags.  (The local binutils seem to work, but they set the ELF
# header "OS/ABI" (EI_OSABI) field to SysV rather than FreeBSD, so
# there might be other problems.)
#
# The --target option is last because the cross-build of LLVM uses
# --target without an OS version ("-freebsd" vs. "-freebsd12").  This
# makes Clang default to libstdc++ (which no longer exists), and also
# controls other features, like GNU-style symbol table hashing and
# anything predicated on the version number in the __FreeBSD__
# preprocessor macro.
# for tool in clang clang++; do
#   tool_path=/usr/local/bin/${triple}-${tool}
#   cat > "$tool_path" <<EOF
# #!/bin/sh
# exec $tool --sysroot=$sysroot --prefix=${sysroot}/bin "\$@" --target=$triple
# EOF
#   chmod +x "$tool_path"
# done

GCC_VERSION='8.5.0'
# GCC_SUM='d308841a511bb830a6100397b0042db24ce11f642dab6ea6ee44842e5325ed50'
# GCC_BASE="gcc-$GCC_VERSION"
# GCC_TAR="gcc-$GCC_VERSION.tar.xz"
# GCC_URL="https://ftp.gnu.org/gnu/gcc/$GCC_BASE/$GCC_TAR"

GCC_BASE="gcc-releases-gcc-$GCC_VERSION"
GCC_TAR="gcc-$GCC_VERSION.tar.gz"
GCC_URL="https://github.com/gcc-mirror/gcc/archive/refs/tags/releases/$GCC_TAR"

download_file() {
        local file="$1"
        local url="$2"
        local sum="$3"

        while :; do
                if [[ -f "$file" ]]; then
                        if ! h="$(sha256sum "$file" | awk '{ print $1 }')"; then
                                printf 'ERROR: reading hash\n' >&2
                                exit 1
                        fi

                        if [[ "$h" == "$sum" ]]; then
                                return 0
                        fi

                        printf 'WARNING: hash mismatch: %s != expected %s\n' \
                            "$h" "$sum" >&2
                        rm -f "$file"
                fi

                printf 'Downloading: %s\n' "$url"
                if ! curl -f -L -o "$file" "$url"; then
                        rm -f "$file"
                        sleep 1
                fi
        done
}

rm -f /tmp/$GCC_TAR
# download_file "/tmp/$GCC_TAR" "$GCC_URL" "$GCC_SUM"
curl -L "$GCC_URL" -o "/tmp/$GCC_TAR"
rm -rf gcc
mkdir gcc
cd gcc
# tar -xf "/tmp/$GCC_TAR"
tar -xzf "/tmp/$GCC_TAR"
rm -f "/tmp/$GCC_TAR"

cd $GCC_BASE
mkdir build
cd build

# Ancient binutils versions don't understand debug symbols produced by more recent tools.
# Apparently applying `-fPIC` everywhere allows them to link successfully.
export CFLAGS='-fPIC'
export CXXFLAGS='-fPIC'
export CXXFLAGS_FOR_TARGET='-fPIC'
export CFLAGS_FOR_TARGET='-fPIC'
export GCC_NO_EXECUTABLES=1
hide_output ../configure \
  --target="$triple" \
  --prefix="$prefix" \
  --with-sysroot="$sysroot" \
  --with-gnu-as \
  --with-gnu-ld \
  --enable-languages=c,c++ \
  --disable-nls \
  --disable-libgomp \
  --disable-libquadmath \
  --disable-libssp \
  --disable-libvtv \
  --disable-libcilkrts \
  --disable-libada \
  --disable-multilib \
  --disable-libsanitizer \
  --disable-libquadmath-support \
  --disable-shared
  # --disable-lto

hide_output make -j`nproc`
hide_output make install
