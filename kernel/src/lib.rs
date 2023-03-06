// src/lib.rs
use host::{rollup_core::RawRollupCore, runtime::Runtime};
use kernel::kernel_entry;

mod constants;
mod message;
mod stages;

use stages::stage_one;

fn entry<Host: RawRollupCore>(host: &mut Host) {
    host.write_debug("Hello Kernel\n");
    let _messages = stage_one(host);
}

kernel_entry!(entry);
