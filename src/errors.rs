// Copyright 2017 Jonathan Creekmore
//
// Licensed under the MIT license <LICENSE.md or
// http://opensource.org/licenses/MIT>. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(missing_docs)]
error_chain! {
    foreign_links {
        InvalidData(::base64::DecodeError);
        NotUtf8(::std::str::Utf8Error);
    }
    errors {
        MalformedFraming
        MissingBeginTag
        MissingEndTag
        MissingData
        MismatchedTags(b: String, e: String) {
            description("mismatching BEGIN and END tags")
            display("mismatching BEGIN (\"{}\") and END (\"{}\") tags", b, e)
        }
    }
}
