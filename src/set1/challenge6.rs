use bitwise::bitwiseops;
use bitwise::base64;
use challengeinfo::challenge::{Challenge, ChallengeInfo};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::String;
use cryptanalysis::break_xor_cipher;


pub const FILE: &'static str = "data/set1/ex6.txt";

pub const INFO6: ChallengeInfo<'static> = ChallengeInfo {
    set_number: 6,
    challenge_number: 6,
    title: "Break repeating-key XOR",
    description: "",
    url: "http://cryptopals.com/sets/1/challenges/6",
};

pub const CHALLENGE6: Challenge<'static> = Challenge {
    info: INFO6,
    func: execute,
};

fn execute() -> String {
    let keysizes: Vec<usize> = (2..40).collect();

    let handle = File::open(FILE).unwrap();
    let reader = BufReader::new(handle);
    let mut buffer = Vec::<u8>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        buffer.extend_from_slice(line.as_bytes());
    } 

    buffer = base64::from_base64(String::from_utf8(buffer).unwrap().as_str()).unwrap();

    let guessed_keysize = break_xor_cipher::guess_key_size(keysizes.as_ref(), buffer.as_ref());

    String::from("")
}
