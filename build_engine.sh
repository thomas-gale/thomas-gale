#!/bin/bash
cargo build --profile wasm-release --bin engine --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_engine --out-dir target/wasm-bindgen/release --no-typescript --target web target/wasm32-unknown-unknown/wasm-release/engine.wasm
