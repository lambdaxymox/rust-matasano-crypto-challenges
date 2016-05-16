use bitwise::bitwiseops;
use bitwise::hex_rep::ToHexRep;
use challengeinfo::challenge::{Challenge, ChallengeInfo};


pub const INFO2: ChallengeInfo<'static> = ChallengeInfo {
    set_number: 2,
    challenge_number: 2,
    title: "Fixed XOR",
    description: "",
    url: "http://cryptopals.com/sets/1/challenges/2",
};

pub const CHALLENGE2: Challenge<'static> = Challenge {
    info: INFO2,
    func: execute,
};

fn execute() -> String {
    let hex1  = String::from("1c0111001f010100061a024b53535009181c").to_hex().unwrap();
    let hex2  = String::from("686974207468652062756c6c277320657965").to_hex().unwrap();
    let result   = bitwiseops::exact_block_xor(hex1.as_slice(), hex2.as_slice()).unwrap();
    let expected = String::from("746865206b696420646f6e277420706c6179").to_hex().unwrap();

    assert_eq!(expected, result);

    String::from_utf8(result).unwrap()
}
