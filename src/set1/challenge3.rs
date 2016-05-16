use cryptanalysis::english;
use bitwise::bitwiseops;
use bitwise::hex_rep::ToHexRep;
use challengeinfo::challenge::{Challenge, ChallengeInfo};


pub const INFO3: ChallengeInfo<'static> = ChallengeInfo {
    set_number: 3,
    challenge_number: 3,
    title: "Single-byte XOR cipher",
    description: "",
    url: "http://cryptopals.com/sets/1/challenges/3",
};

pub const CHALLENGE3: Challenge<'static> = Challenge {
    info: INFO3,
    func: execute,
};

fn execute() -> String {
    let ciphertext   = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                              .to_hex().unwrap();
    let guessed_char = english::most_likely_byte(&ciphertext);
    let plaintext    = bitwiseops::xor_with_char(guessed_char, ciphertext.as_ref());
    let expected_plaintext = Vec::from("Cooking MC's like a pound of bacon");

    assert_eq!(expected_plaintext.len(), plaintext.len());
    assert_eq!(expected_plaintext, plaintext);

    String::from_utf8(expected_plaintext).unwrap()
}
