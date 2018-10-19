cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target\wasm32-unknown-unknown\debug\echo_callisto.wasm --out-dir .
