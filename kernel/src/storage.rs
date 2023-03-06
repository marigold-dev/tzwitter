use host::{
    path::{OwnedPath, RefPath},
    rollup_core::RawRollupCore,
    runtime::Runtime,
};

use crate::message::Tweet;
use host::path::*;

const TWEETS: RefPath = RefPath::assert_from(b"/tweets");

/// Compute the path of one tweet
///
/// /tweets/{tweet_hash}
fn tweet_path(tweet: &Tweet) -> Result<OwnedPath, &'static str> {
    let tweet_hash = tweet.hash().to_string();

    let path: Vec<u8> = format!("/{}", &tweet_hash).into();
    let path = OwnedPath::try_from(path).map_err(|_| "invalid tweet path")?;

    concat(&TWEETS, &path).map_err(|_| "invalid tweet path")
}

/// Stores a tweet in the durable state
///
/// It returns the stored tweet
pub fn store_tweet<'a, Host: RawRollupCore + Runtime>(
    host: &mut Host,
    tweet: &'a Tweet,
) -> Result<&'a Tweet, &'static str> {
    let path = tweet_path(tweet)?;
    let data = tweet.as_bytes();

    host.store_write(&path, data, 0)
        .map_err(|_| "error when writing in durable state")
        .map(|_| tweet)
}
