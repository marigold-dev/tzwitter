// src/lib.rs
use host::{rollup_core::RawRollupCore, runtime::Runtime};
use kernel::kernel_entry;

mod constants;
mod core;
mod stages;
mod storage;

use stages::stage_one;

fn entry<Host: RawRollupCore>(host: &mut Host) {
    host.write_debug("Hello Kernel\n");
    let messages = stage_one(host);

    // TODO: extract this into a new stage
    messages.iter().for_each(|_| {
        host.write_debug("Receive a message\n");
    });
}

kernel_entry!(entry);
