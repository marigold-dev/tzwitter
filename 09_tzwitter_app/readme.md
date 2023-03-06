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

Architecture:

The kernel has several stages:
 - read the inbox
 - check signatures
 - applying messages

TODO: benchmark each stages and check if they each take less than 11_000_000_000 ticks to be executed. Otherwise we will have to change this (opiniated) architecture to the following one:
 - read one message, check it signature, applying it
 - do it n times (where n is represent n iteration to reach ~11_000_000_000 ticks)
 - save the inbox and reboot

The first solution is _easier_ to implement in my opinion and easier to read for a developper. Remember that this project is an example to show how to develop a kernel