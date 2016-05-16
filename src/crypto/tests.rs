use super::xor_cipher::{SingleCharXorCipher, BlockCipher};
use std::convert::From;
use std::string::String;

#[test]
fn test_xor_cipher() {
    let plaintext   = Vec::from("Super duper secret text");
    let key         = 'x' as u8;
    let cipher      = SingleCharXorCipher::new(key);
    let ciphertext  = cipher.process_block(plaintext.as_ref());

    let mut expected_ct = Vec::new();

    for ch in &ciphertext {
        expected_ct.push(*ch);
    };

    let mut expected_pt = Vec::new();

    for ch in &ciphertext {
        expected_pt.push(ch ^ key);
    }

    let expected = String::from_utf8(expected_pt).unwrap();

    assert_eq!(String::from_utf8(ciphertext).unwrap(), String::from_utf8((expected_ct)).unwrap());
    assert_eq!(String::from_utf8(plaintext).unwrap(), expected);


}