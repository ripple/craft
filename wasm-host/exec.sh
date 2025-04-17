#!/bin/bash
cargo build
cargo run -- --wasm-file ../projects/xrpl_std_example/target/wasm32-unknown-unknown/debug/xrpl_std_example.wasm --test-case success


