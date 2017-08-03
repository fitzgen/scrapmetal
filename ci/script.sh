#!/usr/bin/env bash

set -eux

case "$JOB" in
    "test")
        cargo build $PROFILE --verbose --features "$FEATURES"
        cargo test  $PROFILE --verbose --features "$FEATURES"
        ;;
    "bench")
        if [[ "$PROFILE" != "--release" ]]; then
            echo Benching a non-release build??
            exit 1
        fi
        cargo bench --verbose --features "$FEATURES"
        ;;
    *)
        echo Unknown job: "$JOB"
        exit 1
        ;;
esac
