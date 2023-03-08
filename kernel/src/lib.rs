// src/lib.rs
use host::{rollup_core::RawRollupCore, runtime::Runtime};
use kernel::kernel_entry;

mod constants;
mod core;
mod stages;
mod storage;

use crate::core::error::*;
use stages::{read_input, verify_signature};

/// A step is processing only one message from the inbox
///
/// It will execute several sub steps:
/// - read the next message from the inbox
/// - verify its signature
/// - interpret the message
/// - save the result to the durable state
fn step<Host: RawRollupCore>(host: &mut Host) -> Result<()> {
    host.write_debug("Processing message\n");
    let message = read_input(host)?;
    host.write_debug("Message is deserialized\n");
    let _inner = verify_signature(message)?;
    host.write_debug("Signature is correct\n");
    Ok(())
}

/// Process all the inbox
///
/// It also has the responsability to reboot the kernel and count ticks
fn execute<Host: RawRollupCore>(host: &mut Host) -> Result<()> {
    let result = step(host);
    match result {
        Ok(()) => Ok(()),
        Err(Error::SerdeJson(_)) => execute(host),
        Err(Error::FromUtf8Error(_)) => execute(host),
        Err(Error::EndOfInbox) => Ok(()),
        Err(Error::NotATzwitterMessage) => execute(host),
        Err(Error::Runtime) => Err(Error::Runtime),
        Err(Error::Ed25519Compact(_)) => execute(host),
        Err(Error::InvalidSignature) => execute(host),
    }
}

fn entry<Host: RawRollupCore>(host: &mut Host) {
    host.write_debug("Hello Kernel\n");

    match execute(host) {
        Ok(()) => {}
        Err(err) => host.write_debug(&err.to_string()),
    }
}

kernel_entry!(entry);
