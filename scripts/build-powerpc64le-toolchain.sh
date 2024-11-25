#!/usr/bin/env bash

set -ex

hide_output() {
  set +x
  on_err="
echo ERROR: An error was encountered with the build.
cat /tmp/build.log
exit 1
"
  trap "$on_err" ERR
  bash -c "while true; do sleep 30; echo \$(date) - building ...; done" &
  PING_LOOP_PID=$!
  "$@" &> /tmp/build.log
  rm /tmp/build.log
  trap - ERR
  kill $PING_LOOP_PID
  set -x
}

BINUTILS=2.32
GCC=8.3.0
TARGET=powerpc64le-linux-gnu
TRIPLE=powerpc64le-unknown-linux-gnu
PREFIX="/x-tools/$TRIPLE"
SYSROOT="$PREFIX/sysroot"

# First, download the CentOS7 glibc.ppc64le and relevant header files.
# (upstream ppc64le support wasn't added until 2.19, which el7 backported.)
sudo mkdir -p $SYSROOT
sudo chown `whoami` $SYSROOT
sudo chmod u+w $SYSROOT
sudo mkdir -p $PREFIX
sudo chown `whoami` $PREFIX
sudo chmod u+w $PREFIX
sudo chmod u+w /x-tools
pushd $SYSROOT

# centos_base=http://vault.centos.org/altarch/7.3.1611/os/ppc64le/Packages/
# Mirrored from centos_base above
centos_base=https://ci-mirrors.rust-lang.org/rustc
glibc_v=2.17-157-2020-11-25.el7
kernel_v=3.10.0-514-2020-11-25.el7
for package in glibc{,-devel,-headers}-$glibc_v kernel-headers-$kernel_v; do
  curl $centos_base/$package.ppc64le.rpm | \
    rpm2cpio - | cpio -idm
done

ln -sT lib64 lib
ln -sT lib64 usr/lib

popd

# Next, download and build binutils.
mkdir binutils-$TARGET
pushd binutils-$TARGET
curl https://ftp.gnu.org/gnu/binutils/binutils-$BINUTILS.tar.xz | tar xJf -
mkdir binutils-build
export CFLAGS='-Os -s'
export CXXFLAGS='-Os -s'
export LDFLAGS='-s'
cd binutils-build
hide_output ../binutils-$BINUTILS/configure --target=$TARGET --with-sysroot=$SYSROOT --prefix=$PREFIX
hide_output make -j`nproc`
hide_output make install
popd
rm -rf binutils-$TARGET

# Finally, download and build gcc.
mkdir gcc-$TARGET
pushd gcc-$TARGET
curl https://ftp.gnu.org/gnu/gcc/gcc-$GCC/gcc-$GCC.tar.xz | tar xJf -
cd gcc-$GCC
hide_output ./contrib/download_prerequisites

mkdir ../gcc-build
cd ../gcc-build
export CFLAGS="-Os -s"
export CXXFLAGS="-Os -s"
export LDFLAGS="-s"
hide_output ../gcc-$GCC/configure                            \
  --enable-languages=c,c++                       \
  --target=$TARGET                               \
  --with-cpu=power8                              \
  --with-sysroot=$SYSROOT                        \
  --disable-libcilkrts                           \
  --disable-multilib                             \
  --disable-nls                                  \
  --disable-libgomp                              \
  --disable-libquadmath                          \
  --disable-libssp                               \
  --disable-libvtv                               \
  --disable-libcilkrt                            \
  --disable-libada                               \
  --disable-libsanitizer                         \
  --disable-libquadmath-support                  \
  --with-gnu-as                                  \
  --with-gnu-ld                                  \
  --prefix=$PREFIX                               \
  --enable-lto


hide_output make -j`nproc`
make install

popd
rm -rf gcc-$TARGET
