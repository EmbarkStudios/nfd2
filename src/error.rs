use std::{ffi, fmt, str};

#[derive(Debug)]
pub enum NFDError {
    NulError(ffi::NulError),
    Utf8Error(str::Utf8Error),
    Error(String),
}

impl fmt::Display for NFDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NFDError::NulError(ref err) => err.fmt(f),
            NFDError::Error(ref err) => err.fmt(f),
            NFDError::Utf8Error(ref err) => err.fmt(f),
        }
    }
}

impl From<ffi::NulError> for NFDError {
    fn from(err: ffi::NulError) -> NFDError {
        NFDError::NulError(err)
    }
}

impl From<str::Utf8Error> for NFDError {
    fn from(err: str::Utf8Error) -> NFDError {
        NFDError::Utf8Error(err)
    }
}
