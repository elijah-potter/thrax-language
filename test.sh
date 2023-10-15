#!/bin/bash

set -eo pipefail

cd ./crates

cargo test
cargo test --release
