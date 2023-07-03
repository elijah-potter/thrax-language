#!/bin/bash

set -eo pipefail

START_DIR=$(pwd)

WASM_PACK_FLAG="--dev"
CARGO_FLAG=""
DO_TEST=false

for arg in "$@"; do
  case $arg in 
    -r | --release)
      WASM_PACK_FLAG="--release"
      CARGO_FLAG=""
      ;;
    -d | --debug)
      WASM_PACK_FLAG="--dev"
      CARGO_FLAG=""
      ;;
    -t | --test)
      DO_TEST=true
      ;;
  esac
done

# Build normal crates normally
for crate in ast cli interpreter parser; do
  cd $START_DIR/crates/$crate
  cargo build $CARGO_FLAG
  if $DO_TEST; then
    cargo test $CARGO_FLAG
  fi
done

# Build WebAssembly
cd $START_DIR/crates/wasm
wasm-pack build $CARGO_FLAG

cd $START_DIR/packages/demo
yarn
yarn build
