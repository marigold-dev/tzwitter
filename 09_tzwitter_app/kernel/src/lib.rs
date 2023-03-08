// src/lib.rs
use host::{rollup_core::RawRollupCore, runtime::Runtime};
use kernel::kernel_entry;

mod constants;
mod core;
mod stages;
mod storage;

use stages::read_input;

fn execute<Host: RawRollupCore>(host: &mut Host) -> Result<(), &'static str> {
    let message = read_input(host)?;
    match message {
        None => Ok(()),
        Some(_message) => {
            host.write_debug("Processing message");

            // TODO: verify signature
            // TODO: interpret the message
            // TODO: save the message to the durable state

            // TODO: check if reboot is required
            execute(host)
        }
    }
}

fn entry<Host: RawRollupCore>(host: &mut Host) {
    host.write_debug("Hello Kernel\n");

    match execute(host) {
        Ok(()) => {}
        Err(err) => host.write_debug(err),
    }
}

kernel_entry!(entry);
