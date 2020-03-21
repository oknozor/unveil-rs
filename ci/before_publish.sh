#!/usr/bin/env bash

set -ex

# Stash release archives before publishing to crates.io
git add .
git stash