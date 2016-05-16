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

#[test]
fn test_score_with() {
    use num_rational::Ratio;
    use std::collections::HashMap;
    use super::frequency_analysis::Histogram;
    use std::convert::From;

    fn make_table(array: &[(u8, usize)]) -> (usize, HashMap<u8,usize>) {
        let mut table = HashMap::new();
        let mut total = 0;

        for ref kv in array {
            table.insert(kv.0, kv.1);
            total += kv.1;
        }

        (total, table)
    }

    let plaintext = Vec::from("Cooking MC\'s like a pound of bacon");
    let (total, table): (usize, HashMap<u8,usize>)
         = make_table(english::FREQUENCY_LIST.as_ref());

    let model = Histogram::from(english::FREQUENCY_LIST.as_ref());
    let score = break_xor_cipher::score_with(&model, plaintext.as_ref());

    let mut expected_score = Ratio::from_integer(0);
    for ch in plaintext.into_iter() {
        if table.contains_key(&ch) {
            expected_score = expected_score + Ratio::new_raw(*table.get(&ch).unwrap(), total);
        }
    }

    assert_eq!(expected_score, score);
}
