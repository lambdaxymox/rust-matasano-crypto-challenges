use std::string::String;
use bitwise::hex_rep::ToHexRep;
use bitwise::bitwiseops;
use cryptanalysis::english;
use std::fs::File;


#[test]
fn test_challenge1() {
    let hex_str  = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let result   = hex_str.to_hex().unwrap();
    let expected: Vec<u8> 
        = vec![0x49, 0x27, 0x6d, 0x20, 0x6b, 0x69, 0x6c, 0x6c, 0x69, 0x6e, 
               0x67, 0x20, 0x79, 0x6f, 0x75, 0x72, 0x20, 0x62, 0x72, 0x61,
               0x69, 0x6e, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x61, 0x20,
               0x70, 0x6f, 0x69, 0x73, 0x6f, 0x6e, 0x6f, 0x75, 0x73, 0x20,
               0x6d, 0x75, 0x73, 0x68, 0x72, 0x6f, 0x6f, 0x6d
            ];

    assert_eq!(expected.len(), result.len());
    assert_eq!(expected, result);
}

#[test]
fn test_challenge2() {
    let hex1     = String::from("1c0111001f010100061a024b53535009181c").to_hex().unwrap();
    let hex2     = String::from("686974207468652062756c6c277320657965").to_hex().unwrap();
    let result   = bitwiseops::exact_block_xor(hex1.as_slice(), hex2.as_slice()).unwrap();
    let expected = String::from("746865206b696420646f6e277420706c6179").to_hex().unwrap();

    assert_eq!(expected.len(), result.len());
    assert_eq!(expected, result);
}

#[test]
fn test_challenge3() {
    let ciphertext = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                              .to_hex().unwrap();
    let expected_plaintext = Vec::from("Cooking MC's like a pound of bacon");
    let key                = 0x58 as u8;
    let guessed_byte       = english::most_likely_byte(ciphertext.as_slice());
    let guessed_plaintext  = bitwiseops::xor_with_char(guessed_byte, ciphertext.as_slice());

    assert_eq!(key, guessed_byte);
    assert_eq!(expected_plaintext.len(), guessed_plaintext.len());
    assert_eq!(expected_plaintext, guessed_plaintext);
}

#[test]
fn test_challenge4_file_open() {
    let handle = File::open("data/set1/ex4.txt");    

    assert!(handle.is_ok());
}

#[test]
fn test_challenge5() {
    let plaintext  = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let ciphertext = String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f")
                      .to_hex().unwrap();
    let key        = "ICE";
    let result     = bitwiseops::xor_with_key(key.as_ref(), plaintext.as_ref());

    assert_eq!(result.len(), ciphertext.len());
    assert_eq!(result.as_slice(), ciphertext.as_slice());
}