#!/bin/sh

export RUST_BACKTRACE=1
TARGET=i686-unknown-linux-gnu
cd api && cargo +nightly expand --target $TARGET > ../export.rs && cd ..
cd api && cargo +nightly build --target $TARGET && cd ..
cd target/$TARGET/debug && objcopy -L __rust_alloc -L __rust_dealloc -L __rust_realloc \
    -L __rust_alloc_zeroed -L __rust_alloc_error_handler libapi.a libapi_stripped.a && cd ../../..
cd native && cargo +nightly expand --target $TARGET > ../import.rs && cd ..
cd native && cargo +nightly run --target $TARGET && cd ..
