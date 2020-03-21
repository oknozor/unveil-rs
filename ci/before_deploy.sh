#!/usr/bin/env bash
# Building and packaging for release
set -ex

# Ensure environment variables exist.
if [[ -z "$PROJECT_NAME" ]]; then
    export PROJECT_NAME="unveil"
fi

build() {
    echo "Building unveil for target arch $TARGET"
    cargo build --bin unveil --target "$TARGET" --release --verbose
}

build_pages() {
    echo "Building unveil demo project before publishing to github pages"
    cd demo
    cargo run --target "$TARGET" --release -- build
    cd ..
}

pack() {
    echo "Packaging release files and .deb"
    local tempdir
    local out_dir
    local package_name
    local gcc_prefix

    tempdir=$(mktemp -d 2>/dev/null || mktemp -d -t tmp)
    out_dir=$(pwd)
    package_name="$PROJECT_NAME-$TRAVIS_TAG-$TARGET"

    if [[ $TARGET == "arm-unknown-linux-gnueabihf" ]]; then
        gcc_prefix="arm-linux-gnueabihf-"
    else
        gcc_prefix=""
    fi

    # create a "staging" directory
    mkdir "$tempdir/$package_name"

    # copying the main binary
    cp "target/$TARGET/release/$PROJECT_NAME" "$tempdir/$package_name/"
    "${gcc_prefix}"strip "$tempdir/$package_name/$PROJECT_NAME"

    # readme and license
    cp README.md "$tempdir/$package_name"
    cp LICENSE "$tempdir/$package_name"


    # TODO: autocomplete

    # archiving
    pushd "$tempdir"
    tar czf "$out_dir/$package_name.tar.gz" "$package_name"/*
    popd
    rm -r "$tempdir"
}

make_deb() {
    local tempdir
    local architecture
    local version
    local dpkgname
    local conflictname
    local gcc_prefix
    local homepage
    local maintainer

    homepage="https://github.com/oknozor/unveil-rs"
    maintainer="Paul Delafosse <paul.delafosse@protonmail.com>"

    version=${TRAVIS_TAG#v}
    if [[ $TARGET = *musl* ]]; then
      dpkgname=$PROJECT_NAME-musl
      conflictname=$PROJECT_NAME
    else
      dpkgname=$PROJECT_NAME
      conflictname=$PROJECT_NAME-musl
    fi

    tempdir=$(mktemp -d 2>/dev/null || mktemp -d -t tmp)

    # copy the main binary
    install -Dm755 "target/$TARGET/release/$PROJECT_NAME" "$tempdir/usr/bin/$PROJECT_NAME"
    "${gcc_prefix}"strip "$tempdir/usr/bin/$PROJECT_NAME"

    # readme and license
    install -Dm644 README.md "$tempdir/usr/share/doc/$PROJECT_NAME/README.md"
    install -Dm644 LICENSE "$tempdir/usr/share/doc/$PROJECT_NAME/LICENSE-MIT"
    cat > "$tempdir/usr/share/doc/$PROJECT_NAME/copyright" <<EOF
Format: http://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: $PROJECT_NAME
Source: $homepage
Files: *
Copyright: $maintainer
License: MIT
 Permission is hereby granted, free of charge, to any
 person obtaining a copy of this software and associated
 documentation files (the "Software"), to deal in the
 Software without restriction, including without
 limitation the rights to use, copy, modify, merge,
 publish, distribute, sublicense, and/or sell copies of
 the Software, and to permit persons to whom the Software
 is furnished to do so, subject to the following
 conditions:
 .
 The above copyright notice and this permission notice
 shall be included in all copies or substantial portions
 of the Software.
 .
 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
 ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
 TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
 PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
 SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
 IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 DEALINGS IN THE SOFTWARE.
EOF

    # Control file
    mkdir "$tempdir/DEBIAN"
    cat > "$tempdir/DEBIAN/control" <<EOF
Package: $dpkgname
Version: $version
Section: utils
Priority: optional
Maintainer: Paul Delafosse <paul.delafosse@protonmail.com>
Architecture: $architecture
Provides: $PROJECT_NAME
Conflicts: $conflictname
Description: A tool to create presentations from markdown files.
EOF

    fakeroot dpkg-deb --build "$tempdir" "${dpkgname}_${version}_${architecture}.deb"
}


main() {
    # This is a very hacky way to bypass travis deploy lifecycle limitation
    # We shall find a better way

    if [[ $DEPLOY_TARGET == "github-pages" ]]; then
      build_pages
    fi

    if [[ $DEPLOY_TARGET == "github-pages" && $DEPLOY_TARGET == "cargo-publish" ]]; then
      build
      pack
      if [[ $TARGET = *linux* ]]; then
        make_deb
      fi
    fi
}

main