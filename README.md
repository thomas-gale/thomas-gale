# tgale.eth
Personal website

## dev
- [install rust](https://www.rust-lang.org/tools/install)
- [yew setup](https://yew.rs/docs/getting-started/introduction)
- [bevy wasm setup](https://github.com/bevyengine/bevy/tree/latest/examples#wasm)

### build bevy part (release)
```shell
cargo build --profile wasm-release --bin engine --target wasm32-unknown-unknown
```
```shell
wasm-bindgen --out-name wasm_engine --out-dir target/wasm-bindgen/release --target web target/wasm32-unknown-unknown/wasm-release/engine.wasm
```

#### [wasm-opt](https://archlinux.org/packages/community/x86_64/binaryen/)
```shell
wasm-opt -Oz --output target/wasm-bindgen/release/optimized.wasm target/wasm-bindgen/release/wasm_engine_bg.wasm
mv target/wasm-bindgen/release/optimized.wasm target/wasm-bindgen/release/wasm_engine_bg.wasm
```

### build/run yew part
```shell
trunk serve
```

## deploy (this triggered automatically by [fleek](https://app.fleek.co/) and deployed to IPNS/IPFS)
(this command is here for reference)
```shell
trunk build --release
```
 