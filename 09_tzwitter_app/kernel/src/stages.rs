use crate::{
    constants::{L1_TOKEN_CONTRACT_ADDRESS, L1_TOKEN_CONTRACT_ENTRYPOINT, MAGIC_BYTE},
    core::{
        account::Account,
        message::{Content, Inner, PostTweet, Transfer},
        nonce::Nonce,
        tweet::Tweet,
    },
    storage::{
        self, add_collecting_tweet_to_account, add_owned_tweet_to_account,
        add_written_tweet_to_account, increment_tweet_counter, is_liked, is_not_collected,
        is_owner, read_tweet, set_collected_block, set_like_flag, store_tweet,
    },
};
use host::{
    rollup_core::{RawRollupCore, MAX_INPUT_MESSAGE_SIZE},
    runtime::Runtime,
};
use num_bigint::ToBigInt;
use tezos_data_encoding::{enc::BinWriter, types::Zarith};

use crate::core::error::*;
use crate::core::message::Message;
use tezos_rollup_encoding::{contract::Contract, inbox::InboxMessage, michelson::*};
use tezos_rollup_encoding::{entrypoint::Entrypoint, outbox::*};

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

/// Returns the hash of the previous block
/// /!\ /!\ This function should be call one time BEFORE the read_input function (see above)
/// Because it will read the first 2 messages of the inbox
/// TODO: how to have the current level of the block?
/// It should be better if this function returns the current level of the block
pub fn get_previous_block_hash<Host: RawRollupCore + Runtime>(host: &mut Host) -> Result<String> {
    // It ignores the StartOfLevel
    let _ = host
        .read_input(MAX_INPUT_MESSAGE_SIZE)
        .map_err(|err| Error::Runtime(err))?;

    // It reads the InfoPerLevel
    let input = host
        .read_input(MAX_INPUT_MESSAGE_SIZE)
        .map_err(|err| Error::Runtime(err))?;

    // And then extract the precessor hash as a string (which not the best type)

    let input = input.ok_or(Error::NotInfoPerLevelMsg)?;
    let data = input.as_ref();
    let msg = InboxMessage::<MichelsonUnit>::parse(data)
        .map_err(|_| Error::NotInfoPerLevelMsg)?
        .1;

    match msg {
        InboxMessage::External(_) => Err(Error::NotInfoPerLevelMsg),
        InboxMessage::Internal(msg) => match msg {
            tezos_rollup_encoding::inbox::InternalInboxMessage::InfoPerLevel(info) => {
                Ok(info.predecessor.to_base58_check())
            }
            _ => Err(Error::NotInfoPerLevelMsg),
        },
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

/// Withdraw the tweet to layer 1
pub fn withdraw_tweet<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    previous_hash: &str,
    account: &Account,
    tweet_id: &u64,
) -> Result<()> {
    let () = is_owner(host, &account.public_key_hash, tweet_id)?;
    let () = is_not_collected(host, tweet_id)?;

    let tweet = read_tweet(host, tweet_id)
        .map_err(Error::from)?
        .ok_or(Error::TweetNotFound)?;

    let owner = {
        let contract = Contract::from_b58check(&account.public_key_hash.to_string())
            .map_err(|_| Error::FromBase58CheckError)?;
        MichelsonContract(contract)
    };
    let author = {
        let contract = Contract::from_b58check(&tweet.author.to_string())
            .map_err(|_| Error::FromBase58CheckError)?;
        MichelsonContract(contract)
    };
    let id = {
        let id = tweet_id.to_bigint().ok_or(Error::BigIntError)?;
        let id = Zarith(id);
        MichelsonInt(id)
    };
    // What to do with that?
    let likes = {
        let likes = tweet.likes.to_bigint().ok_or(Error::BigIntError)?;
        let likes = Zarith(likes);
        MichelsonInt(likes)
    };
    let content = MichelsonString(tweet.content);

    let destination = Contract::from_b58check(L1_TOKEN_CONTRACT_ADDRESS)
        .map_err(|_| Error::FromBase58CheckError)?;

    // (pair %mint
    //     (pair (nat %id) (address %owner))
    //     (pair %token (pair (address %author) (string %content)) (nat %likes)))

    let michelson = MichelsonPair(
        MichelsonPair(id, owner),
        MichelsonPair(MichelsonPair(author, content), likes),
    );

    let transaction = OutboxMessageTransaction {
        parameters: michelson,
        destination,
        entrypoint: Entrypoint::try_from(L1_TOKEN_CONTRACT_ENTRYPOINT.to_string())
            .map_err(Error::from)?,
    };

    let batch = OutboxMessageTransactionBatch::from(vec![transaction]);
    let message = OutboxMessage::AtomicTransactionBatch(batch);

    let mut output = Vec::default();
    let () = message.bin_write(&mut output).unwrap();

    let () = host.write_output(&output).unwrap();

    // Freeze the tweets
    let () = set_collected_block(host, tweet_id, previous_hash)?;
    // Indicates that the user is collecting the tweet
    let () = add_collecting_tweet_to_account(host, &account.public_key_hash, tweet_id)?;
    Ok(())
}
