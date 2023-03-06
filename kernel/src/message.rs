use crate::core::hash::Blake2b;

pub struct Tweet(pub String);

pub enum Message {
    Tweet(Tweet),
}

impl Tweet {
    /// Hash the tweet
    pub fn hash(&self) -> Blake2b {
        let Tweet(string) = self;
        let bytes = string.as_bytes();
        Blake2b::from(bytes)
    }

    /// Convert a tweet as bytes
    ///
    /// Used for storing it
    /// TODO: should we extract it into a Trait?
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
