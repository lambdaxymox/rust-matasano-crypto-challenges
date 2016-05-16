use super::break_xor_cipher;
use super::english;
use std::convert::From;
use crypto::xor_cipher::SingleCharXorCipher;
use crypto::xor_cipher::BlockCipher;

#[test]
fn test_most_likely_byte() {
    let plaintext  = Vec::from("Cooking MC\'s like a pound of bacon");
    let key        = 0x58 as u8;
    let cipher     = SingleCharXorCipher::new(key);
    let ciphertext = cipher.process_block(plaintext.as_ref());

    let guessed_key = english::most_likely_byte(ciphertext.as_slice());
    let guessed_cipher = SingleCharXorCipher::new(guessed_key);
    let guessed_plaintext = guessed_cipher.process_block(ciphertext.as_slice());

    assert_eq!(key, guessed_key);
    assert_eq!(plaintext.as_slice(), guessed_plaintext.as_slice());
}
