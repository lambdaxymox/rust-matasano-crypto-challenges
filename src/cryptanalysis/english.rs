#![allow(dead_code)]
use std::convert::From;
use super::frequency_analysis::Histogram;
use super::break_xor_cipher;
use num_rational::Ratio;


pub const FREQUENCY_LIST: [(u8, usize); 53] = 
        [(b'E', 1041442), (b'e', 1041442), (b'T', 0729357), (b't', 0729357),
         (b'A', 0651738), (b'a', 0651738), (b'O', 0596302), (b'o', 0596302),
         (b'I', 0558094), (b'i', 0558094), (b'N', 0564513), (b'n', 0564513),
         (b'S', 0515760), (b's', 0515760), (b'H', 0492888), (b'h', 0492888),
         (b'R', 0497563), (b'r', 0497563), (b'D', 0349835), (b'd', 0349835),
         (b'L', 0331490), (b'l', 0331490), (b'C', 0217339), (b'c', 0217339),
         (b'U', 0225134), (b'u', 0225134), (b'M', 0202124), (b'm', 0202124),
         (b'W', 0171272), (b'w', 0171272), (b'F', 0197881), (b'f', 0197881),
         (b'G', 0158610), (b'g', 0158610), (b'Y', 0145984), (b'y', 0145984),
         (b'P', 0137645), (b'p', 0137645), (b'B', 0124248), (b'b', 0124248),
         (b'V', 0082903), (b'v', 0082903), (b'K', 0050529), (b'k', 0050529),
         (b'J', 0009033), (b'j', 0009033), (b'X', 0013692), (b'x', 0013692),
         (b'Q', 0008606), (b'q', 0008606), (b'Z', 0007836), (b'z', 0007836),
         (b' ', 2*1918182)
        ];

pub fn most_likely_byte(cipher_text: &[u8]) -> u8 {
    // Converting the frequency table to a histogram every time to score a string is really expensive,
    // but I don't know of a better way to do it.
    // TODO: Find a better way to do this.
    let frequency_table = Histogram::from(FREQUENCY_LIST.as_ref());
    let charset = (0x00..0xFF).collect::<Vec<u8>>();

    break_xor_cipher::break_xor_char(&frequency_table, charset.as_ref(), cipher_text).0
}

pub fn most_likely_char(cipher_text: &str) -> u8 {
    // Converting the frequency table to a histogram every time to score a string is really expensive,
    // but I don't know of a better way to do it.
    // TODO: Find a better way to do this.
    let frequency_table = Histogram::from(FREQUENCY_LIST.as_ref());
    let charset = (0x00..0xFF).collect::<Vec<u8>>();

    break_xor_cipher::break_xor_char(&frequency_table, charset.as_ref(), cipher_text.as_ref()).0
}

pub fn score(cipher_text: &[u8]) -> Ratio<usize> {
    let model = Histogram::from(FREQUENCY_LIST.as_ref());

    break_xor_cipher::score_with(&model, cipher_text)
}
