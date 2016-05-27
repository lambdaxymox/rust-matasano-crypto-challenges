use rustc_serialize::base64;
use rustc_serialize::base64::{ToBase64, FromBase64, FromBase64Error};
use std::convert::AsRef;


fn config_base64() -> base64::Config {
    base64::Config {
        char_set: base64::CharacterSet::Standard,
        newline: base64::Newline::LF,
        pad: false,
        line_length: None,
    }
}

pub fn to_base64(string: &str) -> String {
    let slice: &[u8] = string.as_ref();

    (*slice).to_base64(config_base64())
}

pub fn from_base64(string: &str) -> Result<Vec<u8>, FromBase64Error> {
    let slice: &[u8] = string.as_ref();

    slice.from_base64()
}