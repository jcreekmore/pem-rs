// Copyright 2017 Jonathan Creekmore
//
// Licensed under the MIT license <LICENSE.md or
// http://opensource.org/licenses/MIT>. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(missing_docs)]
error_chain! {
    foreign_links {
        InvalidData(::rustc_serialize::base64::FromBase64Error);
    }
    errors {
        MalformedFraming
        MissingTag(t: String) {
            description("missing tag")
            display("missing \"{}\" tag", t)
        }
        MismatchedTags(b: String, e: String) {
            description("mismatching BEGIN and END tags")
            display("mismatching BEGIN (\"{}\") and END (\"{}\") tags", b, e)
        }
    }
}
