# Tzitter

The project is divided into two sub projects:
 - the kernel, that is ran by the rollup
 - the front, that can be uploaded anywhere


## The kernel



The kernel is developped with Rust and compiled to Wasm.

You will need rust 1.66 and the wasm32-unknown-unknown target installed

```bash
$ cargo --manifest-path kernel/Cargo.toml build --release --target wasm32-unknown-unknown
```