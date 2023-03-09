#!/bin/sh

account_alias=$1
if [[ "$account_alias" == "" ]]; then echo "first argument should be an account alias with enough tez" && exit 1;fi

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
KERNEL=$(xxd -ps -c 0 rollup/kernel.wasm | tr -d '\n')

# Setup the DAC
mkdir -p rollup/wasm_2_0_0
mkdir -p /tmp/dac
octez-dac-node configure as legacy with threshold 0 and data availability committee members --data-dir /tmp/dac --reveal-data-dir $PWD/rollup/wasm_2_0_0
echo "{ \"reveal_data_dir\": \"$PWD/rollup/wasm_2_0_0\", \"mode\": { \"legacy\": true } }" > /tmp/dac/config.json

# Run the DAC
octez-dac-node run --data-dir /tmp/dac &> /dev/null&
dac_pid=$!

# Split the kernel
sleep 3; # To be sure the dac is started
ROOT_HASH=$(echo "{\"payload\": \"$KERNEL\", \"pagination_scheme\":\"Merkle_tree_V0\"}" | curl --silent --header "Content-Type: application/json" -X POST -d @- http://localhost:10832/store_preimage | jq -r ".root_hash");

# Installer kernel
# To be faster we clone only one time the kernel repository
if [ ! -d "/tmp/kernel" ]; then
  git clone git@gitlab.com:tezos/kernel.git /tmp/kernel
fi

# Modify the line 41 of the installer_kernel (TODO: find a regex to do so, that does not implu )
cd /tmp/kernel
sed -i "66s/.*/b\"${ROOT_HASH}\";/" /tmp/kernel/installer_kernel/src/lib.rs
cargo make wasm-preimage-installer 

wasm-strip /tmp/kernel/target/wasm32-unknown-unknown/release/tezos_rollup_installer_kernel.wasm
KERNEL_INSTALLER=$(xxd -ps -c 0 /tmp/kernel/target/wasm32-unknown-unknown/release/tezos_rollup_installer_kernel.wasm | tr -d '\n')
cd -
ls

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