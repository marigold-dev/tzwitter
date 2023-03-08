use crate::{
    constants::MAGIC_BYTE,
    core::{
        message::{Content, Inner, PostTweet},
        nonce::Nonce,
        tweet::Tweet,
    },
    storage::store_tweet,
};
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
    let input = host
        .read_input(MAX_INPUT_MESSAGE_SIZE)
        .map_err(Error::from)?;
    match input {
        None => Err(Error::EndOfInbox),
        Some(message) => {
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

/// Verify the nonce of the inner message
///
/// If the nonce is correct the content of the inner is returned
pub fn verify_nonce(inner: Inner, nonce: &Nonce) -> Result<Content> {
    let next_nonce = nonce.next();
    let inner_nonce = inner.nonce();
    if &next_nonce == inner_nonce {
        let Inner { content, .. } = inner;
        Ok(content)
    } else {
        Err(Error::InvalidNonce)
    }
}

/// Create a new tweet from the PostTweet request
/// Save the tweet to the durable state
pub fn create_tweet<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    post_tweet: PostTweet,
) -> Result<()> {
    let tweet = Tweet::from(post_tweet);
    let _ = store_tweet(host, &tweet)?;
    Ok(())
}
