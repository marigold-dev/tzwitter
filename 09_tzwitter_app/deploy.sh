#!/bin/sh

# check account
account_alias=$1
if [[ "$account_alias" == "" ]]; then echo "first argument should be an account alias with enough tez" && exit 1;fi

# check if smart-rollup-installer is installed
which smart-rollup-installer > /dev/null || (echo "smart-rollup-installer should be installed" && echo "cargo install tezos_smart_rollup_installer --git https://gitlab.com/tezos/tezos" && exit 1)

# xxd should be installed
which xxd > /dev/null || (echo "xxd should be installed" && exit 1)

# wasm-strip
which wasm-strip > /dev/null || (echo "wasm-strip should be installed" && echo "https://github.com/WebAssembly/wabt" exit 1)

# Compiling the kernel
cargo build --release --target wasm32-unknown-unknown --manifest-path kernel/Cargo.toml

# Deletes the previous rollup
rm -rf rollup

# Copy the kernel in the rollup directory
mkdir -p rollup
cp kernel/target/wasm32-unknown-unknown/release/tzwitter_kernel.wasm ./rollup/kernel.wasm

# Installing the kernel
wasm-strip ./rollup/kernel.wasm

# Split it 
smart-rollup-installer get-reveal-installer --upgrade-to rollup/kernel.wasm --output rollup/installer.hex --preimages-dir rollup/wasm_2_0_0

# Setup the DAC
mkdir -p rollup/wasm_2_0_0

# Copy the kernel in the rollup directory
mkdir -p rollup
cp kernel/target/wasm32-unknown-unknown/release/tzwitter_kernel.wasm ./rollup/kernel.wasm

# Save the bytes of the kernel as a variable
KERNEL_INSTALLER=$(cat rollup/installer.hex)

# Originate the kernel 
SOR_ADDR=$(octez-client originate smart rollup from $account_alias \
  of kind wasm_2_0_0 \
  of type bytes \
  with kernel "${KERNEL_INSTALLER}" \
  --burn-cap 999 | grep "Address:" | awk '{print $2}')

# # Setting up the rollup data directory
octez-smart-rollup-node-alpha init operator config for "${SOR_ADDR}" with operators "$account_alias" --data-dir rollup

# # Print the function to run it
echo Rollup Address: $SOR_ADDR
echo Command to run to start your rollup:
echo "octez-smart-rollup-node-alpha run --data-dir rollup"