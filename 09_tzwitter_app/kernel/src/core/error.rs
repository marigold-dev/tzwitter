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
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::SerdeJson(_) => "Cannot deserialize the message".to_string(),
            Error::EndOfInbox => "End of the inbox".to_string(),
            Error::NotATzwitterMessage => "Not a Tzwitter message".to_string(),
            Error::FromUtf8Error(_) => "Cannot convert bytes to string".to_string(),
            Error::Runtime(_) => "Runtime error, caused by host function".to_string(),
            Error::Ed25519Compact(_) => "Cannot deserialize Ed25519".to_string(),
            Error::InvalidSignature => "Invalid signature".to_string(),
            Error::InvalidNonce => "Invalid nonce".to_string(),
            Error::PathError(_) => "Invalid path".to_string(),
            Error::StateDeserializarion => "State deserialization".to_string(),
            Error::TweetNotFound => "Tweet not found".to_string(),
            Error::TweetAlreadyLiked => {
                "The tweet has already been liked by this account".to_string()
            }
        }
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
