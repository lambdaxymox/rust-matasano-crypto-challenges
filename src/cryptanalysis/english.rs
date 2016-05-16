#![allow(dead_code)]
use std::convert::From;
use super::frequency_analysis::Histogram;
use super::break_xor_cipher;


pub const FREQUENCY_LIST: [(u8, usize); 52] = 
        [('E' as u8, 12702), ('e' as u8, 12702), 
         ('T' as u8, 09056), ('t' as u8, 09056),
         ('A' as u8, 08167), ('a' as u8, 08167),
         ('O' as u8, 07507), ('o' as u8, 07507),
         ('I' as u8, 06966), ('i' as u8, 06966), 
         ('N' as u8, 06749), ('n' as u8, 06749),
         ('S' as u8, 06327), ('s' as u8, 06327),
         ('H' as u8, 06094), ('h' as u8, 06094),
         ('R' as u8, 05987), ('r' as u8, 05987),
         ('D' as u8, 04253), ('d' as u8, 04253),
         ('L' as u8, 04025), ('l' as u8, 04025),
         ('C' as u8, 02782), ('c' as u8, 02782),
         ('U' as u8, 02758), ('u' as u8, 02758), 
         ('M' as u8, 02406), ('m' as u8, 02406),
         ('W' as u8, 02361), ('w' as u8, 02361),
         ('F' as u8, 02228), ('f' as u8, 02228),
         ('G' as u8, 02015), ('g' as u8, 02015),
         ('Y' as u8, 01974), ('y' as u8, 01974),
         ('P' as u8, 01929), ('p' as u8, 01929),
         ('B' as u8, 01492), ('b' as u8, 01492),
         ('V' as u8, 00978), ('v' as u8, 00978),
         ('K' as u8, 00772), ('k' as u8, 00772),
         ('J' as u8, 00153), ('j' as u8, 00153),
         ('X' as u8, 00150), ('x' as u8, 00150),
         ('Q' as u8, 00095), ('q' as u8, 00095),
         ('Z' as u8, 00074), ('z' as u8, 00074)
        ];

pub fn most_likely_byte(cipher_text: &[u8]) -> u8 {
    // Converting the frequency table to a histogram every time to score a string is really expensive,
    // but I don't know of a better way to do it.
    // TODO: Find a better way to do this.
    let frequency_table = Histogram::from(FREQUENCY_LIST.as_ref());

    break_xor_cipher::break_xor_char(&frequency_table, &(0x00..0xFF).collect::<Vec<u8>>().as_ref(), cipher_text).0
}

pub fn most_likely_char(cipher_text: &str) -> u8 {
    // Converting the frequency table to a histogram every time to score a string is really expensive,
    // but I don't know of a better way to do it.
    // TODO: Find a better way to do this.
    let frequency_table = Histogram::from(FREQUENCY_LIST.as_ref());

    break_xor_cipher::break_xor_char(&frequency_table, &(0x00..0xFF).collect::<Vec<u8>>().as_ref(), cipher_text.as_ref()).0
}
