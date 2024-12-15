use core::str::Utf8Error;
use std::error::Error;
use std::fmt::Display;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Marc8Error {
    Utf8Error(Utf8Error),
    FromUtf8Error(FromUtf8Error),
    NoData,
}

impl Error for Marc8Error {}

impl Display for Marc8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match &self {
            Self::Utf8Error(_) | Self::FromUtf8Error(_) => "could not decode from utf8",
            Self::NoData => "no data in char buffer",
        };
        write!(f, "{message}")
    }
}

impl From<Utf8Error> for Marc8Error {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

impl From<FromUtf8Error> for Marc8Error {
    fn from(value: FromUtf8Error) -> Self {
        Self::FromUtf8Error(value)
    }
}
