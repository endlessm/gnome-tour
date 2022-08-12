#!/bin/bash
export DIST="$1"
export SOURCE_ROOT="$2"

# Endless OS has prepared ".cargo/config" and "vendor" for source replacement
test -f "$DIST/.cargo/config" && exit

cd "$SOURCE_ROOT"
mkdir "$DIST"/.cargo
cargo vendor | sed 's/^directory = ".*"/directory = "vendor"/g' > $DIST/.cargo/config
# Move vendor into dist tarball directory
mv vendor "$DIST"

