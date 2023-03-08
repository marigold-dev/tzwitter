use serde::Serialize;

use crate::core::public_key_hash::PublicKeyHash;

use super::{hash::Blake2b, message::PostTweet};

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

impl Tweet {
    /// Hash of the tweet
    pub fn hash(&self) -> Blake2b {
        let to_hash = format!("{}{}", self.author.to_b58(), &self.content);
        Blake2b::from(to_hash.as_bytes())
    }
}
