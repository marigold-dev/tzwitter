/// Rperesents the error of the read_input functions
#[derive(Debug)]
pub enum ReadInputError {
    /// The message does not be process by this rollup
    NotATzwitterMessage,
    /// There is no more messages
    EndOfInbox,
    /// There is an error in the bytes to string deserialization
    FromUtf8Error(std::string::FromUtf8Error),
    /// There is an error in the string to Message deserialization
    SerdeJson(serde_json_wasm::de::Error),
    /// There is an error runtime
    Runtime(host::runtime::RuntimeError),
}

/// Represents all the error of the kernel
///
#[derive(Debug)]
pub enum Error {
    FromUtf8Error(std::string::FromUtf8Error),
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
register_error!(Ed25519Compact, ed25519_compact::Error);
register_error!(PathError, host::path::PathError);
register_error!(Runtime, host::runtime::RuntimeError);

pub type Result<A> = std::result::Result<A, Error>;
