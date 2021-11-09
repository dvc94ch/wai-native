#!/bin/sh

export RUST_BACKTRACE=1
cd api && cargo +nightly expand --target wasm32-wasi > ../export.rs && cd ..
cd api && cargo +nightly build --target wasm32-wasi  && cd ..
cd wasm && cargo +nightly expand > ../import.rs && cd ..
cd wasm && cargo +nightly run && cd ..
