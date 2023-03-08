use crate::constants::MAGIC_BYTE;
use host::{
    rollup_core::{RawRollupCore, MAX_INPUT_MESSAGE_SIZE},
    runtime::Runtime,
};

use crate::core::message::Message;

/**
 * It will recursively read the inbox of the rollup
 * Parse the messages
 */
fn aux_stage_one<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    mut inbox: Vec<Message>,
) -> Vec<Message> {
    let input = host.read_input(MAX_INPUT_MESSAGE_SIZE);
    match input {
        Err(_) => inbox, // Should we failwith
        Ok(None) => inbox,
        Ok(Some(message)) => {
            let data = message.as_ref();
            match data {
                [0x01, MAGIC_BYTE, ..] => {
                    let bytes = data.iter().skip(2).copied().collect(); // Skip first and magic byte.
                    let str = String::from_utf8(bytes); // JSON string
                    match str {
                        Err(_) => aux_stage_one(host, inbox),
                        Ok(string) => {
                            // aux_stage_one(host, inbox)
                            let str = serde_json_wasm::from_str(&string);
                            match str {
                                Err(_) => aux_stage_one(host, inbox),
                                Ok(msg) => {
                                    inbox.push(msg);
                                    aux_stage_one(host, inbox)
                                }
                            }
                        }
                    }
                }
                _ => aux_stage_one(host, inbox),
            }
        }
    }
}

/**
 * Parse the inbox into a list of messages
 */
pub fn stage_one<Host: RawRollupCore>(host: &mut Host) -> Vec<Message> {
    aux_stage_one(host, Vec::default())
}
