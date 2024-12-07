use core::str::Utf8Error;
use std::error::Error;
use std::fmt::Display;
use std::string::FromUtf16Error;

#[derive(Debug)]
pub enum EncodingError {
    Utf8Error(Utf8Error),
    Utf16Error(FromUtf16Error),
    NoData,
}

impl Error for EncodingError {}

impl Display for EncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match &self {
            Self::Utf8Error(_) => "could not decode from utf8",
            Self::Utf16Error(_) => "could not decode from utf16",
            Self::NoData => "no data in char buffer",
        };
        write!(f, "{message}")
    }
}

impl From<Utf8Error> for EncodingError {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

impl From<FromUtf16Error> for EncodingError {
    fn from(value: FromUtf16Error) -> Self {
        Self::Utf16Error(value)
    }
}
