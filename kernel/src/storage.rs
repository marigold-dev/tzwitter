use crate::core::public_key_hash::PublicKeyHash;
use crate::core::receipt::Receipt;
use crate::core::tweet::Tweet;
use crate::core::{account::Account, error::*, nonce::Nonce};
use host::path::Path;
use host::runtime::{load_value_sized, load_value_slice};
use host::{
    path::{concat, OwnedPath, RefPath},
    rollup_core::RawRollupCore,
    runtime::Runtime,
};

const ACCOUNTS: RefPath = RefPath::assert_from(b"/accounts");
pub const TWEETS: RefPath = RefPath::assert_from(b"/tweets");
const TWEET_COUNTER: RefPath = RefPath::assert_from(b"/constants/tweet-counter"); // The name constants is not appropriate
const RECEIPTS: RefPath = RefPath::assert_from(b"/receipts");

/// Compute the paths for the different fields of a tweet
///
/// The field_path should start with slash
fn tweet_field_path(tweet_id: &u64, field_path: &str) -> Result<OwnedPath> {
    let tweet_path: Vec<u8> = format!("/{}", tweet_id).into();
    let tweet_path = OwnedPath::try_from(tweet_path).map_err(Error::from)?;
    let tweet_path = concat(&TWEETS, &tweet_path).map_err(Error::from)?;

    let field_path: Vec<u8> = field_path.into();
    let field_path = OwnedPath::try_from(field_path).map_err(Error::from)?;
    concat(&tweet_path, &field_path).map_err(Error::from)
}

/// Compute the path of the tweet author
/// /tweets/{id}/author
fn tweet_author_path(tweet_id: &u64) -> Result<OwnedPath> {
    tweet_field_path(tweet_id, "/author")
}

/// Compute the path of the tweet content
/// /tweets/{hash}/content
fn tweet_content_path(tweet_id: &u64) -> Result<OwnedPath> {
    tweet_field_path(tweet_id, "/content")
}

/// Compute the path of the tweet content
/// /tweets/{hash}/content
fn tweet_likes_path(tweet_id: &u64) -> Result<OwnedPath> {
    tweet_field_path(tweet_id, "/likes")
}

/// Path to know if user has collected the tweet
/// The stored value is the block level
/// /tweets/{id}/collected_hash
fn tweet_collected_block_path(tweet_id: &u64) -> Result<OwnedPath> {
    tweet_field_path(tweet_id, "/collected_hash")
}

/// Compute the paths for the different fields of an account
///
/// The field_path should start with slash
fn account_field_path(public_key_hash: &PublicKeyHash, field_path: &str) -> Result<OwnedPath> {
    let public_key_hash: Vec<u8> = format!("/{}", public_key_hash.to_string()).into();
    let public_key_hash = OwnedPath::try_from(public_key_hash).map_err(Error::from)?;
    let public_key_hash = concat(&ACCOUNTS, &public_key_hash).map_err(Error::from)?;

    let field_path: Vec<u8> = field_path.into();
    let field_path = OwnedPath::try_from(field_path).map_err(Error::from)?;
    concat(&public_key_hash, &field_path).map_err(Error::from)
}

/// Compute the path /accounts/{tz1...}/nonce
fn nonce_path(public_key_hash: &PublicKeyHash) -> Result<OwnedPath> {
    account_field_path(public_key_hash, "/nonce")
}

/// Compute the path to the liked tweet
fn account_likes_path(public_key_hash: &PublicKeyHash, tweet_id: &u64) -> Result<OwnedPath> {
    account_field_path(public_key_hash, &format!("/likes/{}", tweet_id))
}

/// Compute the path of the being collected tweets
fn account_collecting_path(public_key_hash: &PublicKeyHash, tweet_id: &u64) -> Result<OwnedPath> {
    account_field_path(public_key_hash, &format!("/collecting/{}", tweet_id))
}

/// Path to keep track of owned tweets
///
/// /account/{tz1...}/tweets/{tweet_id}
/// If the id is present in the subkey /tweets then the account owns the tweets
///
/// TODO: this structure is not the best one, it does not ensure that a tweet is owned by only one user.
fn account_owned_tweet_path(public_key_hash: &PublicKeyHash, tweet_id: &u64) -> Result<OwnedPath> {
    account_field_path(public_key_hash, &format!("/tweets/owned/{}", tweet_id))
}

/// Path to keep track of the tweets written by a user
fn account_written_tweet_path(
    public_key_hash: &PublicKeyHash,
    tweet_id: &u64,
) -> Result<OwnedPath> {
    account_field_path(public_key_hash, &format!("/tweets/written/{}", tweet_id))
}

/// Compute the path of the different field of a receipt
fn receipt_field_path(receipt: &Receipt, field_path: &str) -> Result<OwnedPath> {
    let receipt_path = format!("/{}", receipt.hash().to_string());
    let receipt_path = OwnedPath::try_from(receipt_path).map_err(Error::from)?;
    let receipt_path = concat(&RECEIPTS, &receipt_path)?;

    let field_path: Vec<u8> = field_path.into();
    let field_path = OwnedPath::try_from(field_path).map_err(Error::from)?;
    concat(&receipt_path, &field_path).map_err(Error::from)
}

/// Compute the path of the success field of a receipt
fn receipt_success_path(receipt: &Receipt) -> Result<OwnedPath> {
    receipt_field_path(receipt, "/success")
}

///  Check if a path exists
pub fn exists<Host: RawRollupCore + Runtime>(host: &mut Host, path: &impl Path) -> Result<bool> {
    let exists = Runtime::store_has(host, path)?
        .map(|_| true)
        .unwrap_or_default();
    Ok(exists)
}

/// Read an u64 from a given path
/// If the data does not exist, it returns the default value of an u64
pub fn read_u64<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    path: &impl Path,
) -> Result<Option<u64>> {
    let is_exists = exists(host, path)?;
    if !is_exists {
        return Ok(None);
    }

    let mut buffer = [0_u8; 8];
    match load_value_slice(host, path, &mut buffer) {
        Ok(8) => Ok(Some(u64::from_be_bytes(buffer))),
        _ => Err(Error::StateDeserializarion),
    }
}

/// Store an u64 at a given path
fn store_u64<'a, Host: RawRollupCore + Runtime>(
    host: &mut Host,
    path: &impl Path,
    u64: &'a u64,
) -> Result<&'a u64> {
    let data = u64.to_be_bytes();
    let data = data.as_slice();

    host.store_write(path, data, 0)
        .map_err(Error::from)
        .map(|_| u64)
}

/// Stores a string at a given path
fn store_string<'a, Host: RawRollupCore + Runtime, T>(
    host: &mut Host,
    path: &OwnedPath,
    data: &'a T,
) -> Result<&'a T>
where
    T: ToString,
{
    let string = data.to_string();
    let bytes = string.as_bytes();
    host.store_write(path, bytes, 0)
        .map_err(Error::from)
        .map(|_| data)
}

fn read_string<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    path: &OwnedPath,
) -> Result<Option<String>> {
    let is_exists = exists(host, path)?;
    if !is_exists {
        return Ok(None);
    }

    let buffer = load_value_sized(host, path).map_err(Error::from)?;
    String::from_utf8(buffer)
        .map_err(Error::from)
        .map(|str| Some(str))
}

/// Creates a flag at the given path
fn store_flag<Host: RawRollupCore + Runtime>(host: &mut Host, path: &impl Path) -> Result<()> {
    let data = [0x00].as_slice();
    host.store_write(path, data, 0)
        .map_err(Error::from)
        .map(|_| ())
}

/// Stores a boolean at a given path
fn store_bool<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    path: &impl Path,
    bool: bool,
) -> Result<()> {
    let data = match bool {
        true => [0x01],
        false => [0x00],
    };

    host.store_write(path, &data, 0)
        .map_err(Error::from)
        .map(|_| ())
}

/// Read the account of the user
pub fn read_account<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    public_key_hash: PublicKeyHash,
) -> Result<Account> {
    let nonce_path = nonce_path(&public_key_hash)?;
    let nonce = read_u64(host, &nonce_path)?.unwrap_or_default();
    Ok(Account {
        public_key_hash,
        nonce: Nonce(nonce),
    })
}

/// Store an account to the location /account/{tz...}
///
/// Only the nonce is stored
pub fn store_account<'a, Host: RawRollupCore + Runtime>(
    host: &mut Host,
    account: &'a Account,
) -> Result<&'a Account> {
    let Account {
        nonce,
        public_key_hash,
    } = account;
    let nonce_path = nonce_path(public_key_hash)?;
    let _ = store_u64(host, &nonce_path, &nonce.0)?;
    Ok(account)
}

/// Store a tweet to the location /tweets/{tz...}
pub fn store_tweet<'a, Host: RawRollupCore + Runtime>(
    host: &mut Host,
    tweet_id: &u64,
    tweet: &'a Tweet,
) -> Result<&'a Tweet> {
    let Tweet {
        author,
        content,
        likes,
    } = tweet;
    let author_path = tweet_author_path(tweet_id)?;
    let content_path = tweet_content_path(tweet_id)?;
    let likes_path = tweet_likes_path(tweet_id)?;

    let _ = store_string(host, &author_path, author)?;
    let _ = store_string(host, &content_path, content)?;
    let _ = store_u64(host, &likes_path, likes)?;

    Ok(tweet)
}

/// Increment the tweet counter and return the previous one.
pub fn increment_tweet_counter<Host: RawRollupCore + Runtime>(host: &mut Host) -> Result<u64> {
    let previous_counter = read_u64(host, &TWEET_COUNTER)?.unwrap_or_default();
    let next_counter = previous_counter + 1;
    let _ = store_u64(host, &TWEET_COUNTER, &next_counter)?;
    Ok(previous_counter)
}

/// Read a tweet from the durable state
///
/// If the tweet is not present an Option is return
pub fn read_tweet<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    tweet_id: &u64,
) -> Result<Option<Tweet>> {
    let author_path = tweet_author_path(tweet_id)?;
    let content_path = tweet_content_path(tweet_id)?;
    let likes_path = tweet_likes_path(tweet_id)?;

    let author = read_string(host, &author_path)?;
    let author = match author {
        None => None,
        Some(str) => Some(PublicKeyHash::from_b58(&str)?),
    };
    let content = read_string(host, &content_path)?;
    let likes = read_u64(host, &likes_path)?;

    match (author, content, likes) {
        (Some(author), Some(content), Some(likes)) => Ok(Some(Tweet {
            author,
            content,
            likes,
        })),
        _ => Ok(None),
    }
}

/// Create a flag in the user account that indicates that the user has liked the given tweet
pub fn set_like_flag<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    public_key_hash: &PublicKeyHash,
    tweet_id: &u64,
) -> Result<()> {
    let path = account_likes_path(public_key_hash, tweet_id)?;
    store_flag(host, &path)
}

/// Check if the user has a like a tweet
pub fn is_liked<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    public_key_hash: &PublicKeyHash,
    tweet_id: &u64,
) -> Result<bool> {
    let path = account_likes_path(public_key_hash, tweet_id)?;
    exists(host, &path)
}

/// Add a tweet in the "written" path of an account
pub fn add_written_tweet_to_account<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    public_key_hash: &PublicKeyHash,
    tweet_id: &u64,
) -> Result<()> {
    let path = account_written_tweet_path(public_key_hash, tweet_id)?;
    store_flag(host, &path)
}

/// Add a tweet in the "owned" path of an account
pub fn add_owned_tweet_to_account<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    public_key_hash: &PublicKeyHash,
    tweet_id: &u64,
) -> Result<()> {
    let path = account_owned_tweet_path(public_key_hash, tweet_id)?;
    store_flag(host, &path)
}

/// Checks if the user is owner of the tweet
pub fn is_owner<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    public_key_hash: &PublicKeyHash,
    tweet_id: &u64,
) -> Result<()> {
    let path = account_owned_tweet_path(public_key_hash, tweet_id)?;
    let is_present = exists(host, &path)?;

    match is_present {
        true => Ok(()),
        false => Err(Error::NotOwner),
    }
}

/// Transfer a tweet from a user to another one
/// Does not check if the user owns the tweet
pub fn transfer<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    public_key_hash: &PublicKeyHash,
    tweet_id: &u64,
    destination: &PublicKeyHash,
) -> Result<()> {
    let from = account_owned_tweet_path(public_key_hash, tweet_id)?;
    let to = account_owned_tweet_path(destination, tweet_id)?;
    host.store_move(&from, &to).map_err(Error::from)
}

// Stores a receipt under /receipt/{hash}
pub fn store_receipt<'a, Host: RawRollupCore + Runtime>(
    host: &mut Host,
    receipt: &'a Receipt,
) -> Result<&'a Receipt> {
    let success_path = receipt_success_path(receipt)?;

    let () = store_bool(host, &success_path, receipt.success())?;

    Ok(receipt)
}

/// Returns Ok if the tweet is not collected
pub fn is_not_collected<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    tweet_id: &u64,
) -> Result<()> {
    let tweet_collected_block_path = tweet_collected_block_path(tweet_id)?;
    let is_present = exists(host, &tweet_collected_block_path)?;

    match is_present {
        true => Err(Error::TweetAlreadyCollected),
        false => Ok(()),
    }
}

/// Set the block when the tweet has been collected
pub fn set_collected_block<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    tweet_id: &u64,
    previous_block: &str,
) -> Result<()> {
    let tweet_collected_block_path = tweet_collected_block_path(tweet_id)?;
    let _ = store_string(host, &tweet_collected_block_path, &previous_block)?;
    Ok(())
}

/// Indicates that a tweet is beeing collected by the given user
pub fn add_collecting_tweet_to_account<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    public_key_hash: &PublicKeyHash,
    tweet_id: &u64,
) -> Result<()> {
    let account_collecting_path = account_collecting_path(public_key_hash, tweet_id)?;
    store_flag(host, &account_collecting_path)
}
