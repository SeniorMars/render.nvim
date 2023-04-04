#!/usr/bin/env bash

set -e

# get current version from Cargo.toml
get_version() {
  grep '^version =' Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/'
}

# compile from source
build() {
  echo "Building render.nvim from source..."

  cargo build --release --target-dir ./target

  # Place the compiled library where Neovim can find it.
  mkdir -p lua

  if [ "$(uname)" == "Darwin" ]; then
    mv target/release/librender.dylib lua/typst.so
  elif [ "$(cut -c 1-5 <<< $(uname -s))" == "Linux" ]; then
    mv target/release/librender.so lua/typst.so
  elif [ "$(cut -c 1-10 <<< $(uname -s))" == "MINGW64_NT" ]; then
    mv target/release/render.dll lua/typst.dll
  fi
}

case "$1" in
  build)
    build
    ;;
  *)
    version="v$(get_version)"
    echo "Unknown command: $1"
    ;;
esac
