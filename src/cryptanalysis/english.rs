#![allow(dead_code)]
use std::convert::From;
use super::frequency_analysis::Histogram;
use super::break_xor_cipher;
use num_rational::Ratio;


pub const FREQUENCY_LIST: [(u8, usize); 53] = 
        [('E' as u8, 1041442), ('e' as u8, 1041442), 
         ('T' as u8, 0729357), ('t' as u8, 0729357),
         ('A' as u8, 0651738), ('a' as u8, 0651738),
         ('O' as u8, 0596302), ('o' as u8, 0596302),
         ('I' as u8, 0558094), ('i' as u8, 0558094), 
         ('N' as u8, 0564513), ('n' as u8, 0564513),
         ('S' as u8, 0515760), ('s' as u8, 0515760),
         ('H' as u8, 0492888), ('h' as u8, 0492888),
         ('R' as u8, 0497563), ('r' as u8, 0497563),
         ('D' as u8, 0349835), ('d' as u8, 0349835),
         ('L' as u8, 0331490), ('l' as u8, 0331490),
         ('C' as u8, 0217339), ('c' as u8, 0217339),
         ('U' as u8, 0225134), ('u' as u8, 0225134), 
         ('M' as u8, 0202124), ('m' as u8, 0202124),
         ('W' as u8, 0171272), ('w' as u8, 0171272),
         ('F' as u8, 0197881), ('f' as u8, 0197881),
         ('G' as u8, 0158610), ('g' as u8, 0158610),
         ('Y' as u8, 0145984), ('y' as u8, 0145984),
         ('P' as u8, 0137645), ('p' as u8, 0137645),
         ('B' as u8, 0124248), ('b' as u8, 0124248),
         ('V' as u8, 0082903), ('v' as u8, 0082903),
         ('K' as u8, 0050529), ('k' as u8, 0050529),
         ('J' as u8, 0009033), ('j' as u8, 0009033),
         ('X' as u8, 0013692), ('x' as u8, 0013692),
         ('Q' as u8, 0008606), ('q' as u8, 0008606),
         ('Z' as u8, 0007836), ('z' as u8, 0007836),
         (' ' as u8, 2*1918182)
        ];

pub fn most_likely_byte(cipher_text: &[u8]) -> u8 {
    // Converting the frequency table to a histogram every time to score a string is really expensive,
    // but I don't know of a better way to do it.
    // TODO: Find a better way to do this.
    let frequency_table = Histogram::from(FREQUENCY_LIST.as_ref());
    let charset = (0x00..0xFF).collect::<Vec<u8>>();

    break_xor_cipher::break_xor_char(&frequency_table, &charset.as_ref(), cipher_text).0
}

pub fn most_likely_char(cipher_text: &str) -> u8 {
    // Converting the frequency table to a histogram every time to score a string is really expensive,
    // but I don't know of a better way to do it.
    // TODO: Find a better way to do this.
    let frequency_table = Histogram::from(FREQUENCY_LIST.as_ref());
    let charset = (0x00..0xFF).collect::<Vec<u8>>();

    break_xor_cipher::break_xor_char(&frequency_table, &charset.as_ref(), cipher_text.as_ref()).0
}

pub fn score(cipher_text: &[u8]) -> Ratio<usize> {
    let model = Histogram::from(FREQUENCY_LIST.as_ref());

    break_xor_cipher::score_with(&model, cipher_text)
}
