use serde::Serialize;

use crate::core::public_key_hash::PublicKeyHash;

use super::message::PostTweet;

#[derive(Serialize)]
pub struct Tweet {
    pub author: PublicKeyHash,
    pub content: String,
    pub likes: u64,
}

impl From<PostTweet> for Tweet {
    fn from(post_tweet: PostTweet) -> Self {
        let PostTweet { author, content } = post_tweet;
        Tweet {
            author,
            content,
            likes: 0,
        }
    }
}

impl Tweet {
    pub fn like(self) -> Self {
        Self {
            likes: self.likes + 1,
            ..self
        }
    }
}
