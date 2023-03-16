use crate::{
    constants::MAGIC_BYTE,
    core::{
        account::Account,
        message::{Content, Inner, PostTweet, Transfer},
        nonce::Nonce,
        tweet::Tweet,
    },
    storage::{
        self, add_owned_tweet_to_account, add_written_tweet_to_account, increment_tweet_counter,
        is_liked, is_owner, read_tweet, set_like_flag, store_tweet,
    },
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
pub fn read_input<Host: RawRollupCore>(
    host: &mut Host,
) -> std::result::Result<Message, ReadInputError> {
    let input = host
        .read_input(MAX_INPUT_MESSAGE_SIZE)
        .map_err(|err| ReadInputError::Runtime(err))?;
    match input {
        None => Err(ReadInputError::EndOfInbox),
        Some(message) => {
            let data = message.as_ref();
            match data {
                [0x01, MAGIC_BYTE, ..] => {
                    let bytes = data.iter().skip(2).copied().collect();
                    let str = String::from_utf8(bytes)
                        .map_err(|err| ReadInputError::FromUtf8Error(err))?;
                    serde_json_wasm::from_str(&str).map_err(|err| ReadInputError::SerdeJson(err))
                }
                _ => Err(ReadInputError::NotATzwitterMessage),
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
/// And add a tweet entry to the user account
pub fn create_tweet<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    account: &Account,
    post_tweet: PostTweet,
) -> Result<()> {
    let id = increment_tweet_counter(host)?;
    let tweet = Tweet::from(post_tweet);
    let _ = store_tweet(host, &id, &tweet)?;
    let _ = add_owned_tweet_to_account(host, &account.public_key_hash, &id)?;
    let _ = add_written_tweet_to_account(host, &account.public_key_hash, &id)?;
    Ok(())
}

pub fn like_tweet<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    account: &Account,
    tweet_id: &u64,
) -> Result<()> {
    let already_liked = is_liked(host, &account.public_key_hash, tweet_id)?;
    match already_liked {
        true => Err(Error::TweetAlreadyLiked),
        false => {
            let tweet = read_tweet(host, tweet_id)?;
            match tweet {
                None => Err(Error::TweetNotFound),
                Some(tweet) => {
                    let tweet = tweet.like();
                    store_tweet(host, tweet_id, &tweet)?;
                    let _ = set_like_flag(host, &account.public_key_hash, &tweet_id)?;
                    Ok(())
                }
            }
        }
    }
}

/// Transfer a tweet from an account to another one
///
/// Checks if the account parameter is owner of the tweet
pub fn transfer_tweet<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    account: &Account,
    transfer: &Transfer,
) -> Result<()> {
    let Transfer {
        tweet_id,
        destination,
    } = transfer;
    let () = is_owner(host, &account.public_key_hash, tweet_id)?;
    let () = storage::transfer(host, &account.public_key_hash, tweet_id, destination)?;
    Ok(())
}
