#!/bin/sh

export MESON_BUILD_ROOT="$1"
export MESON_SOURCE_ROOT="$2"
export CARGO_TARGET_DIR="$MESON_BUILD_ROOT"/target
export CARGO_HOME="$MESON_BUILD_ROOT"/cargo-home
FEATURES="$6"

if [ "$4" = "Devel" ]
then
    echo "DEBUG MODE"
    cargo build --manifest-path \
        "$MESON_SOURCE_ROOT"/Cargo.toml $FEATURES && \
        cp "$CARGO_TARGET_DIR"/debug/$5 $3
else
    echo "RELEASE MODE"
    cargo build --manifest-path \
        "$MESON_SOURCE_ROOT"/Cargo.toml $FEATURES --release && \
        cp "$CARGO_TARGET_DIR"/release/$5 $3
fi

