use bitwise::base64;
use bitwise::hex_rep::ToHexRep;
use challengeinfo::challenge::{Challenge, ChallengeInfo};


pub const INFO1: ChallengeInfo<'static> = ChallengeInfo {
    set_number: 1,
    challenge_number: 1,
    title: "Convert hex to base64",
    description: "",
    url: "http://cryptopals.com/sets/1/challenges/1",
};

pub const CHALLENGE1: Challenge<'static> = Challenge {
    info: INFO1,
    func: execute,
};

fn execute() -> String {
    let hex_str  = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let result   = base64::to_base64(&String::from_utf8(hex_str.to_hex().unwrap()).unwrap());

    result
}
