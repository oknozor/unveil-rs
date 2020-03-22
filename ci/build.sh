#!/usr/bin/env bash
set -ex

cargo build --target "$TARGET" --verbose

# We cannot run arm executables on linux
if [[ $TARGET != arm-unknown-linux-gnueabihf ]] ; then
    cargo test --target "$TARGET" --verbose
    # run unveil commands (except serve)
    cargo run -- --help
    cargo run -- init unveil-project
    cd unveil-project
    cargo run -- build
    cargo run -- add test_slide
    cd .. && rm -rf unveil-project
fi