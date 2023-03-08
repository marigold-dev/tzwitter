use crate::{constants::MAGIC_BYTE, core::message::Inner};
use host::{
    rollup_core::{RawRollupCore, MAX_INPUT_MESSAGE_SIZE},
    runtime::Runtime,
};

use crate::core::error::*;
use crate::core::message::Message;

/// Read a message from the inbox
///
/// It will only read messages External Messages with the MAGIC_BYTE
/// Benchmark: 2_000_000 ticks (processing an inbox with only one message)
pub fn read_input<Host: RawRollupCore>(host: &mut Host) -> Result<Message> {
    let input = host.read_input(MAX_INPUT_MESSAGE_SIZE);
    match input {
        Err(_) => Err(Error::Runtime),
        Ok(None) => Err(Error::EndOfInbox),
        Ok(Some(message)) => {
            let data = message.as_ref();
            match data {
                [0x01, MAGIC_BYTE, ..] => {
                    let bytes = data.iter().skip(2).copied().collect();
                    let str = String::from_utf8(bytes).map_err(Error::from)?;
                    serde_json_wasm::from_str(&str).map_err(Error::from)
                }
                _ => Err(Error::NotATzwitterMessage),
            }
        }
    }
}

/// Verify the signature of a message
///
/// Returns the inner message
pub fn verify_signature(message: Message) -> Result<Inner> {
    let signature = message.signature();
    let pkey = message.public_key();
    let inner = message.inner();
    let hash = inner.hash();

    let () = signature.verify(&pkey, hash.as_ref())?;
    let Message { inner, .. } = message;
    Ok(inner)
}
