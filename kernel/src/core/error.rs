/// Represents all the error of the kernel
///
#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json_wasm::de::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    EndOfInbox,
    NotATzwitterMessage,
    Runtime(host::runtime::RuntimeError),
    Ed25519Compact(ed25519_compact::Error),
    InvalidSignature,
    InvalidNonce,
    PathError(host::path::PathError),
    StateDeserializarion,
    TweetNotFound,
    TweetAlreadyLiked,
    NotOwner,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        let err = match self {
            Error::SerdeJson(_) => "Cannot deserialize the message",
            Error::EndOfInbox => "End of the inbox",
            Error::NotATzwitterMessage => "Not a Tzwitter message",
            Error::FromUtf8Error(_) => "Cannot convert bytes to string",
            Error::Runtime(_) => "Runtime error, caused by host function",
            Error::Ed25519Compact(_) => "Cannot deserialize Ed25519",
            Error::InvalidSignature => "Invalid signature",
            Error::InvalidNonce => "Invalid nonce",
            Error::PathError(_) => "Invalid path",
            Error::StateDeserializarion => "State deserialization",
            Error::TweetNotFound => "Tweet not found",
            Error::TweetAlreadyLiked => "The tweet has already been liked by this account",
            Error::NotOwner => "Not the owner of the tweet",
        };
        err.to_string()
    }
}

macro_rules! register_error {
    ($name:ident, $error:ty) => {
        impl From<$error> for Error {
            fn from(data: $error) -> Self {
                Error::$name(data)
            }
        }
    };
}

register_error!(FromUtf8Error, std::string::FromUtf8Error);
register_error!(SerdeJson, serde_json_wasm::de::Error);
register_error!(Ed25519Compact, ed25519_compact::Error);
register_error!(PathError, host::path::PathError);
register_error!(Runtime, host::runtime::RuntimeError);

pub type Result<A> = std::result::Result<A, Error>;
