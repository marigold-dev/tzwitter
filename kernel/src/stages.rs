use crate::constants::MAGIC_BYTE;
use host::{
    rollup_core::{RawRollupCore, MAX_INPUT_MESSAGE_SIZE},
    runtime::Runtime,
};

use crate::core::message::Message;

/// Read a message from the inbox
///
/// It will only read messages External Messages with the MAGIC_BYTE
/// Benchmark: 2_000_000 ticks (processing an inbox with only one message)
pub fn read_input<Host: RawRollupCore>(host: &mut Host) -> Result<Option<Message>, &'static str> {
    let input = host.read_input(MAX_INPUT_MESSAGE_SIZE);
    match input {
        Err(_) => Err("Cannot read the inbox"),
        Ok(None) => Ok(None),
        Ok(Some(message)) => {
            let data = message.as_ref();
            match data {
                [0x01, MAGIC_BYTE, ..] => {
                    let bytes = data.iter().skip(2).copied().collect();
                    let str = String::from_utf8(bytes);
                    match str {
                        Err(_) => read_input(host), // Maybe we should handle the error in another way
                        Ok(string) => {
                            let str = serde_json_wasm::from_str(&string);
                            match str {
                                Err(_) => read_input(host), // Maybe we should handle the error in another way
                                Ok(msg) => Ok(Some(msg)),
                            }
                        }
                    }
                }
                _ => read_input(host),
            }
        }
    }
}
