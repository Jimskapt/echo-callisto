# Echo Callisto

*An experimentation in Rust to learning the `wasm-bindgen` tool.*

[https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html#basic-usage](https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html#basic-usage)

## Setup

``` sh
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo +nightly install wasm-bindgen-cli
```

## Build

``` sh
cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target\wasm32-unknown-unknown\debug\echo_callisto.wasm --out-dir .
```
