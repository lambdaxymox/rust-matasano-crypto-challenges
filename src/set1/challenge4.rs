use cryptanalysis::english;
use bitwise::bitwiseops;
use bitwise::hex_rep::ToHexRep;
use challengeinfo::challenge::{Challenge, ChallengeInfo};
use std::io::{BufRead, BufReader};
use std::fs::File;
use num_rational::Ratio;


pub const FILE: &'static str = "data/set1/ex4.txt";

pub const INFO4: ChallengeInfo<'static> = ChallengeInfo {
    set_number: 4,
    challenge_number: 4,
    title: "Detect single-character XOR",
    description: "",
    url: "http://cryptopals.com/sets/1/challenges/4",
};

pub const CHALLENGE4: Challenge<'static> = Challenge {
    info: INFO4,
    func: execute,
};

fn execute() -> String {

    let expected_plaintext = Vec::from("Now that the party is jumping\n");

    let handle = File::open(FILE).unwrap();
    let reader = BufReader::new(handle);

    let mut best_score = Ratio::from_integer(0);
    let mut best_ciphertext: Vec<u8> = Vec::new();
    let mut best_char  = 0x00;

    for line in reader.lines() {
        let line                = line.unwrap();
        let possible_ciphertext = line.to_hex().unwrap();
        let possible_char       = english::most_likely_byte(&possible_ciphertext);
        let possible_plaintext  = bitwiseops::xor_with_char(possible_char, possible_ciphertext.as_ref());
        let score               = english::score(&possible_plaintext);

        if score >= best_score {
            best_score = score;
            best_ciphertext = possible_ciphertext;
            best_char = possible_char;
        }
    }

    let best_plaintext = bitwiseops::xor_with_char(best_char, best_ciphertext.as_ref());

    assert_eq!(expected_plaintext.len(), best_plaintext.len());
    assert_eq!(expected_plaintext, best_plaintext);

    String::from_utf8(best_plaintext).unwrap()

}