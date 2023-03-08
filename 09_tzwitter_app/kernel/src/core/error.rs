/// Represents all the error of the kernel
pub enum Error {
    SerdeJson(serde_json_wasm::de::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    EndOfInbox,
    NotATzwitterMessage,
    Runtime, // Too generic
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::SerdeJson(_) => "Cannot deserialize the message".to_string(),
            Error::EndOfInbox => "End of the inbox".to_string(),
            Error::NotATzwitterMessage => "Not a Tzwitter message".to_string(),
            Error::FromUtf8Error(_) => "Cannot convert bytes to string".to_string(),
            Error::Runtime => "Runtime error, caused by host function".to_string(),
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

pub type Result<A> = std::result::Result<A, Error>;
