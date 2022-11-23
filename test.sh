#!/bin/bash

set -eo pipefail

cargo test
cargo test --release
