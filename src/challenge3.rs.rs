fn challenge3() {
    let cipher_text = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                             .to_hex().unwrap();
    let guessed_char = english::most_likely_char(cipher_text.as_slice()).0;
    let plain_text = String::from_utf8(bitwiseops::xor_with_char(guessed_char, cipher_text.as_slice())).unwrap();

    let expected = String::from("Cooking MC's like a pound of bacon");

    assert_eq!(expected.len(), plain_text.len());
    assert_eq!(expected, plain_text);
}