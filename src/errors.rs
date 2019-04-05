// Copyright 2017 Jonathan Creekmore
//
// Licensed under the MIT license <LICENSE.md or
// http://opensource.org/licenses/MIT>. This file may not be
// copied, modified, or distributed except according to those terms.

/// The `pem` error type.
#[derive(Fail, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum PemError {
    #[fail(display = "mismatching BEGIN (\"{}\") and END (\"{}\") tags", _0, _1)]
    MismatchedTags(String, String),
    #[fail(display = "malformed framing")]
    MalformedFraming,
    #[fail(display = "missing BEGIN tag")]
    MissingBeginTag,
    #[fail(display = "missing END tag")]
    MissingEndTag,
    #[fail(display = "missing data")]
    MissingData,
    #[fail(display = "invalid data: {}", _0)]
    InvalidData(#[fail(cause)] ::base64::DecodeError),
    #[fail(display = "invalid utf-8 value: {}", _0)]
    NotUtf8(#[fail(cause)] ::std::str::Utf8Error),
}
