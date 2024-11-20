#!/usr/bin/env bash

set -ex
PREFIX="/x-tools/xxxxxx-unknown-fuchsia"

FUCHSIA_SDK_URL=https://chrome-infra-packages.appspot.com/dl/fuchsia/sdk/core/linux-amd64
FUCHSIA_SDK_ID=version:21.20240610.2.1
FUCHSIA_SDK_SHA256=2d2d057fc3f0404197cced2200f88cbcdaaf5fbf6475955045091f8676791ce7
FUCHSIA_SDK_USR_DIR=${PREFIX}/core-linux-amd64-fuchsia-sdk
CLANG_DOWNLOAD_URL=\
https://chrome-infra-packages.appspot.com/dl/fuchsia/third_party/clang/linux-amd64
CLANG_DOWNLOAD_ID=git_revision:3809e20afc68d7d03821f0ec59b928dcf9befbf4
CLANG_DOWNLOAD_SHA256=3c2c442b61cd9e8f1b567738f6d53cffe11b3fc820e7dae87a82a0859be8f204

function hide_output {
  { set +x; } 2>/dev/null
  on_err="
echo ERROR: An error was encountered with the build.
cat /tmp/build.log
exit 1
"
  trap "$on_err" ERR
  bash -c "while true; do sleep 30; echo \$(date) - building ...; done" &
  PING_LOOP_PID=$!
  "$@" &> /tmp/build.log
  trap - ERR
  kill $PING_LOOP_PID
  set -x
}

install_clang() {
  mkdir -p clang_download
  pushd clang_download > /dev/null

  # Download clang+llvm
  curl -LO "${CLANG_DOWNLOAD_URL}/+/${CLANG_DOWNLOAD_ID}"
  echo "$(echo ${CLANG_DOWNLOAD_SHA256}) ${CLANG_DOWNLOAD_ID}" | sha256sum --check --status
  unzip -qq ${CLANG_DOWNLOAD_ID} -d clang-linux-amd64

  # Other dists currently depend on our Clang... moving into ${PREFIX} for other
  #  dist usage instead of a Fuchsia ${PREFIX} directory
  chmod -R 777 clang-linux-amd64/.
  cp -a clang-linux-amd64/. ${PREFIX}

  # CFLAGS and CXXFLAGS env variables in main Dockerfile handle sysroot linking
  for arch in x86_64 aarch64; do
    for tool in clang clang++; do
      ln -s ${PREFIX}/bin/${tool} ${PREFIX}/bin/${arch}-unknown-fuchsia-${tool}
    done
    ln -s ${PREFIX}/bin/llvm-ar ${PREFIX}/bin/${arch}-unknown-fuchsia-ar
  done

  popd > /dev/null
  rm -rf clang_download
}

install_zircon_libs() {
  mkdir -p zircon
  pushd zircon > /dev/null

  # Download Fuchsia SDK (with Zircon libs)
  curl -LO "${FUCHSIA_SDK_URL}/+/${FUCHSIA_SDK_ID}"
  echo "$(echo ${FUCHSIA_SDK_SHA256}) ${FUCHSIA_SDK_ID}" | sha256sum --check --status
  unzip -qq ${FUCHSIA_SDK_ID} -d core-linux-amd64

  # Moving SDK into Docker's user-space
  mkdir -p ${FUCHSIA_SDK_USR_DIR}
  chmod -R 777 core-linux-amd64/.
  cp -r core-linux-amd64/* ${FUCHSIA_SDK_USR_DIR}

  popd > /dev/null
  rm -rf zircon
}

hide_output install_clang
hide_output install_zircon_libs
