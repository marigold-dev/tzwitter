use crate::core::hash::Blake2b;
use crate::core::nonce::Nonce;
use crate::core::public_key::PublicKey;
use crate::core::public_key_hash::PublicKeyHash;
use crate::core::signature::Signature;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostTweet {
    pub author: PublicKeyHash, // define a new type for public key
    pub content: String,
}

#[derive(Deserialize)]
pub struct Transfer {
    pub tweet_id: u64,
    pub destination: PublicKeyHash,
}

#[derive(Deserialize)]
pub enum Content {
    PostTweet(PostTweet),
    LikeTweet(u64),
    Transfer(Transfer),
    Collect(u64),
}

#[derive(Deserialize)]
pub struct Inner {
    nonce: Nonce,
    pub content: Content,
}

impl Inner {
    /// Returns the nonce of the inner
    pub fn nonce(&self) -> &Nonce {
        &self.nonce
    }
}

#[derive(Deserialize)]
pub struct Message {
    pkey: PublicKey,
    signature: Signature,
    pub inner: Inner,
}

impl Message {
    /// Returns the public key of the message
    pub fn public_key(&self) -> &PublicKey {
        &self.pkey
    }

    /// Returns the signature of the message
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    /// Returns the inner of the message
    pub fn inner(&self) -> &Inner {
        &self.inner
    }

    /// Returns the hash of the message
    pub fn hash(&self) -> Blake2b {
        self.inner.hash()
    }
}

impl Inner {
    /// Hash of the message
    /// This hash is what the client should signed
    pub fn hash(&self) -> Blake2b {
        // The nonce, and content should be hashed
        let Inner { nonce, content } = &self;
        match &content {
            Content::PostTweet(PostTweet { author, content }) => {
                let string = format!("{}{}{}", nonce.to_string(), author.to_string(), content);
                Blake2b::from(string.as_bytes())
            }
            Content::LikeTweet(tweet_id) => {
                let string = format!("{}{}", nonce.to_string(), tweet_id);
                Blake2b::from(string.as_bytes())
            }
            Content::Transfer(transfer) => {
                let string = format!(
                    "{}{}{}",
                    nonce.to_string(),
                    transfer.destination.to_string(),
                    transfer.tweet_id
                );
                Blake2b::from(string.as_bytes())
            }
            Content::Collect(tweet_id) => {
                let string = format!("{}{}", nonce.to_string(), tweet_id);
                Blake2b::from(string.as_bytes())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use super::{Content, Inner, PostTweet};
    use crate::core::{message::Message, nonce::Nonce, public_key::PublicKey};

    #[test]
    fn test_hash() {
        let expected = "933dd79f9935573925f774ad0ac8789560e2489f083eb7dce7289485e3648a2d";
        let author = PublicKey::from_b58("edpkuDMUm7Y53wp4gxeLBXuiAhXZrLn8XB1R83ksvvesH8Lp8bmCfK")
            .unwrap()
            .into();

        let inner = Inner {
            nonce: Nonce::default().next(),
            content: Content::PostTweet(PostTweet {
                author,
                content: "Hello world".to_string(),
            }),
        };

        let hash = inner.hash();
        assert_eq!(expected, hash.to_string());
    }

    fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect()
    }

    #[test]
    fn test_message_deserizaliation() {
        let data = "7b22706b6579223a7b2245643235353139223a226564706b75444d556d375935337770346778654c425875694168585a724c6e385842315238336b737676657348384c7038626d43664b227d2c227369676e6174757265223a7b2245643235353139223a22656473696775316d5243745a71754c7673706378615958565a64734b4b5371486e5865766e726d68315436334471315272384d316769564c7661706944464b365451434579593678797464476e4b675a7956534844566e756237707579353462443179227d2c22696e6e6572223a7b226e6f6e6365223a312c22636f6e74656e74223a7b22506f73745477656574223a7b22617574686f72223a7b22547a31223a22747a315146443957714c575a6d6d4175716e6e545050556a666175697459455764736876227d2c22636f6e74656e74223a2248656c6c6f20776f726c64227d7d7d7d";

        let data = decode_hex(data).unwrap();
        let string = String::from_utf8(data).unwrap();
        let data = serde_json_wasm::from_str::<Message>(&string);
        assert!(data.is_ok());
    }
}
