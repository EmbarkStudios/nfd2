use std::{error, ffi, fmt, str};

#[derive(Debug)]
pub enum NfdError {
    NulError(ffi::NulError),
    Utf8Error(str::Utf8Error),
    Error(String),
}

impl fmt::Display for NfdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::NulError(ref err) => err.fmt(f),
            Self::Error(ref err) => err.fmt(f),
            Self::Utf8Error(ref err) => err.fmt(f),
        }
    }
}

impl From<ffi::NulError> for NfdError {
    fn from(err: ffi::NulError) -> Self {
        Self::NulError(err)
    }
}

impl From<str::Utf8Error> for NfdError {
    fn from(err: str::Utf8Error) -> Self {
        Self::Utf8Error(err)
    }
}

impl error::Error for NfdError {}
