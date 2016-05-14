use std::collections::hash_map::HashMap;
use num_rational::Ratio;
use std::convert::From;
use super::frequency_analysis::Histogram;
use super::break_xor_cipher;


const FREQUENCY_DENOMINATOR: usize = 100_000;

const FREQUENCY_LIST: [(u8, usize); 52] = 
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

const ENGLISH_LETTERS: [u8; 52] = ['A' as u8,'B' as u8,'C' as u8,'D' as u8,'E' as u8,
                                   'F' as u8,'G' as u8,'H' as u8,'I' as u8,'J' as u8,
                                   'K' as u8,'L' as u8,'M' as u8,'N' as u8,'O' as u8,
                                   'P' as u8,'Q' as u8,'R' as u8,'S' as u8,'T' as u8,
                                   'U' as u8,'V' as u8,'W' as u8,'X' as u8,'Y' as u8,
                                   'Z' as u8,'a' as u8,'b' as u8,'c' as u8,'d' as u8,
                                   'e' as u8,'f' as u8,'g' as u8,'h' as u8,'i' as u8,
                                   'j' as u8,'k' as u8,'l' as u8,'m' as u8,'n' as u8,
                                   'o' as u8,'p' as u8,'q' as u8,'r' as u8,'s' as u8,
                                   't' as u8,'u' as u8,'v' as u8,'w' as u8,'x' as u8,
                                   'y' as u8,'z' as u8];

pub fn most_likely_char(cipher_text: &[u8]) -> (u8, Ratio<usize>) {
    // Converting the frequency table to a histogram every time to score a string is really expensive,
    // but I don't know of a better way to do it that compiles.
    // TODO: Find a better way to do this.
    let frequency_table = Histogram::from(FREQUENCY_LIST.as_ref());

    break_xor_cipher::break_xor_char(&frequency_table, &ENGLISH_LETTERS, cipher_text)
}