#!/bin/sh

# Required variable, otherwise the kernel won't compile
export TZWITTER_L1_CONTRACT=KT1BN5u3CeRzVC7UaJYF4ADYtiHFHzqwC8dV

# Build the kernel
cargo build --release --target wasm32-unknown-unknown --manifest-path kernel/Cargo.toml

# Simulate it
octez-smart-rollup-wasm-debugger kernel/target/wasm32-unknown-unknown/release/tzwitter_kernel.wasm --inputs kernel/inputs.json