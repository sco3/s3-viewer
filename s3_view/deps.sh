#!/usr/bin/env -S bash -x 
cargo install trunk
rustup target add wasm32-unknown-unknown

cargo add yew --features csr
cargo add wasm-bindgen