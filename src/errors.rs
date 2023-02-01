// Copyright 2017 Jonathan Creekmore
//
// Licensed under the MIT license <LICENSE.md or
// http://opensource.org/licenses/MIT>. This file may not be
// copied, modified, or distributed except according to those terms.
use core::fmt;
#[cfg(any(feature = "std", test))]
use std::error::Error;

/// The `pem` error type.
#[derive(Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum PemError {
    MismatchedTags(String, String),
    MalformedFraming,
    MissingBeginTag,
    MissingEndTag,
    MissingData,
    InvalidData(::base64::DecodeError),
    NotUtf8(::core::str::Utf8Error),
}

impl fmt::Display for PemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PemError::MismatchedTags(b, e) => {
                write!(f, "mismatching BEGIN (\"{}\") and END (\"{}\") tags", b, e)
            }
            PemError::MalformedFraming => write!(f, "malformedframing"),
            PemError::MissingBeginTag => write!(f, "missing BEGIN tag"),
            PemError::MissingEndTag => write!(f, "missing END tag"),
            PemError::MissingData => write!(f, "missing data"),
            PemError::InvalidData(e) => write!(f, "invalid data: {}", e),
            PemError::NotUtf8(e) => write!(f, "invalid utf-8 value: {}", e),
        }
    }
}

#[cfg(feature = "std")]
impl Error for PemError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            // Errors originating from other libraries.
            PemError::InvalidData(e) => Some(e),
            PemError::NotUtf8(e) => Some(e),
            // Errors directly originating from `pem-rs`.
            _ => None,
        }
    }
}

// FIXME(oliveruv): this could be better. Wanted to just do
// #[cfg(any(feature = "std", test))]
// but then it seems base64::DecodeError doesn't get the Error impl even
// though it's got the same cfg attribute. Guess the test attribute is
// only valid for the current crate?
#[cfg(not(feature = "std"))]
impl PemError {
    #[allow(missing_docs)]
    pub fn source(&self) -> Option<&(dyn core::fmt::Display + 'static)> {
        match self {
            // Errors originating from other libraries.
            PemError::InvalidData(e) => Some(e),
            PemError::NotUtf8(e) => Some(e),
            // Errors directly originating from `pem-rs`.
            _ => None,
        }
    }
}

/// The `pem` result type.
pub type Result<T> = ::core::result::Result<T, PemError>;
