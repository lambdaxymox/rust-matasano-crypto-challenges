use std::collections::hash_map::HashMap;
use num_rational::Ratio;
use std::convert::From;
use super::frequency_analysis::Histogram;


const FREQUENCY_DENOMINATOR: usize = 100_000;

const frequency_list: [(u8, usize); 26] = 
        [('E' as u8, 12702), 
         ('T' as u8, 09056), 
         ('A' as u8, 08167),
         ('O' as u8, 07507),
         ('I' as u8, 06966), 
         ('N' as u8, 06749),
         ('S' as u8, 06327), 
         ('H' as u8, 06094), 
         ('R' as u8, 05987),
         ('D' as u8, 04253), 
         ('L' as u8, 04025), 
         ('C' as u8, 02782),
         ('U' as u8, 02758), 
         ('M' as u8, 02406), 
         ('W' as u8, 02361),
         ('F' as u8, 02228), 
         ('G' as u8, 02015), 
         ('Y' as u8, 01974),
         ('P' as u8, 01929), 
         ('B' as u8, 01492), 
         ('V' as u8, 00978),
         ('K' as u8, 00772), 
         ('J' as u8, 00153), 
         ('X' as u8, 00150),
         ('Q' as u8, 00095), 
         ('Z' as u8, 00074)
        ];

//const frequency_table: Histogram<u8> = Histogram::from(frequency_list.as_ref());

fn score_func(freqs: &Histogram<u8>) -> Ratio<usize> {
    // Converting the frequency table to a histogram every time to score a string is really expensive,
    // but I don't know of a better way to do it that compiles.
    // TODO: Find a better way to do this.
    let frequency_table = Histogram::from(frequency_list.as_ref());
    let new_freqs: Histogram<u8> = Histogram::transpose(&frequency_table, freqs);

    new_freqs.into_iter().fold(Ratio::from_integer(0), |acc, (key, value)| { acc + value})
}

fn score(bst: &[u8]) -> Ratio<usize> {
    score_func(&Histogram::from(bst))
}
