extern crate rustc_serialize;
extern crate regex;

use rustc_serialize::base64::FromBase64;
use regex::Regex;

const PEM_SECTION: &'static str =
    r"(?s)-----BEGIN (?P<begin>.*?)-----\s*(?P<data>.*?)-----END (?P<end>.*?)-----\s*";

#[derive(Debug)]
pub struct Pem {
    pub tag: String,
    pub contents: Vec<u8>,
}

pub fn parse(input: &str) -> Vec<Pem> {
    // Create the PEM section regex
    let re = Regex::new(PEM_SECTION).unwrap();

    // Each time our regex matches a PEM section, we need to decode it.
    re.captures_iter(input).filter_map(|caps| {
        // Verify that the begin section exists
        let tag = match caps.name("begin") {
            Some(t) => t,
            None => {
                return None;
            }
        };

        // as well as the end section
        let tag_end = match caps.name("end") {
            Some(t) => t,
            None => {
                return None;
            }
        };

        // The beginning and the end sections must match
        if tag != tag_end {
            return None;
        }

        // If they did, then we can grab the data section
        let data = match caps.name("data") {
            Some(d) => d,
            None => {
                return None;
            }
        };

        // And decode it from Base64 into a vector of u8
        let contents = match data.replace("\n", "").from_base64() {
            Ok(c) => c,
            Err(_) => {
                return None;
            }
        };

        Some(Pem {
            tag: tag.to_owned(),
            contents: contents,
        })
    }).collect()
}

#[cfg(test)]
mod test {
    const SAMPLE: &'static str =
"-----BEGIN RSA PRIVATE KEY-----
MIIBPQIBAAJBAOsfi5AGYhdRs/x6q5H7kScxA0Kzzqe6WI6gf6+tc6IvKQJo5rQc
dWWSQ0nRGt2hOPDO+35NKhQEjBQxPh/v7n0CAwEAAQJBAOGaBAyuw0ICyENy5NsO
2gkT00AWTSzM9Zns0HedY31yEabkuFvrMCHjscEF7u3Y6PB7An3IzooBHchsFDei
AAECIQD/JahddzR5K3A6rzTidmAf1PBtqi7296EnWv8WvpfAAQIhAOvowIXZI4Un
DXjgZ9ekuUjZN+GUQRAVlkEEohGLVy59AiEA90VtqDdQuWWpvJX0cM08V10tLXrT
TTGsEtITid1ogAECIQDAaFl90ZgS5cMrL3wCeatVKzVUmuJmB/VAmlLFFGzK0QIh
ANJGc7AFk4fyFD/OezhwGHbWmo/S+bfeAiIh2Ss2FxKJ
-----END RSA PRIVATE KEY-----

-----BEGIN RSA PUBLIC KEY-----
MIIBOgIBAAJBAMIeCnn9G/7g2Z6J+qHOE2XCLLuPoh5NHTO2Fm+PbzBvafBo0oYo
QVVy7frzxmOqx6iIZBxTyfAQqBPO3Br59BMCAwEAAQJAX+PjHPuxdqiwF6blTkS0
RFI1MrnzRbCmOkM6tgVO0cd6r5Z4bDGLusH9yjI9iI84gPRjK0AzymXFmBGuREHI
sQIhAPKf4pp+Prvutgq2ayygleZChBr1DC4XnnufBNtaswyvAiEAzNGVKgNvzuhk
ijoUXIDruJQEGFGvZTsi1D2RehXiT90CIQC4HOQUYKCydB7oWi1SHDokFW2yFyo6
/+lf3fgNjPI6OQIgUPmTFXciXxT1msh3gFLf3qt2Kv8wbr9Ad9SXjULVpGkCIB+g
RzHX0lkJl9Stshd/7Gbt65/QYq+v+xvAeT0CoyIg
-----END RSA PUBLIC KEY-----
";

    #[test]
    fn it_works() {
        let pems = super::parse(SAMPLE);
        assert_eq!(pems.len(), 2);
        assert_eq!(pems[0].tag, "RSA PRIVATE KEY");
        assert_eq!(pems[1].tag, "RSA PUBLIC KEY");
    }
}
