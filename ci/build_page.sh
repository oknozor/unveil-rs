#!/usr/bin/env bash

set -ex
# Build the demo presentation
cd demo
cargo run -- build
cd ..