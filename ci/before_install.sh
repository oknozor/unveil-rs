#!/usr/bin/env bash

set -ex

if [ "$TRAVIS_OS_NAME" != linux ]; then
    exit 0
fi

sudo apt-get update

# needed to build deb packages
sudo apt-get install -y fakeroot