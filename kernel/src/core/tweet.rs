use serde::Serialize;

use crate::core::public_key_hash::PublicKeyHash;

use super::message::PostTweet;

#[derive(Serialize)]
pub struct Tweet {
    pub author: PublicKeyHash,
    pub content: String,
}

impl From<PostTweet> for Tweet {
    fn from(post_tweet: PostTweet) -> Self {
        let PostTweet { author, content } = post_tweet;
        Tweet { author, content }
    }
}
