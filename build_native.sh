#!/bin/sh

export RUST_BACKTRACE=1
cd api && cargo +nightly expand > ../export.rs && cd ..
cd api && cargo +nightly build && cd ..
cd target/debug && objcopy -L __rust_alloc -L __rust_dealloc -L __rust_realloc \
    -L __rust_alloc_zeroed -L __rust_alloc_error_handler libapi.a libapi_stripped.a && cd ../..
cd native && cargo +nightly expand > ../import.rs && cd ..
cd native && cargo +nightly run && cd ..
