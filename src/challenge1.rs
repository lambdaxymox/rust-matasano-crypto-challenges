fn challenge1() {
    let hex_str  = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let result   = base64::to_base64(&String::from_utf8(hex_str.to_hex().unwrap()).unwrap());
    let expected = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

    assert_eq!(expected.len(), result.len());
    assert_eq!(expected, result);
}