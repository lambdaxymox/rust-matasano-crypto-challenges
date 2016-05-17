use matasano_crypto_challenges::set1;
use matasano_crypto_challenges::bitwise::base64;
use matasano_crypto_challenges::bitwise::hex_rep::ToHexRep;


#[test]
fn test_challenge1() {
    let hex_str  = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let expected   = base64::to_base64(&String::from_utf8(hex_str.to_hex().unwrap()).unwrap());

    assert_eq!((set1::challenge1::CHALLENGE1.func)(), expected);
}

#[test]
fn test_challenge2() {
    let expected = String::from_utf8(String::from("746865206b696420646f6e277420706c6179")
                          .to_hex().unwrap())
                          .unwrap();

    assert_eq!((set1::challenge2::CHALLENGE2.func)(), expected);
}

#[test]
fn test_challenge3() {
    let expected = String::from_utf8(Vec::from("Cooking MC's like a pound of bacon")).unwrap();

    assert_eq!((set1::challenge3::CHALLENGE3.func)(), expected);
}

#[test]
fn test_challenge4() {
    let expected = "Now that the party is jumping\n";

    assert_eq!((set1::challenge4::CHALLENGE4.func)(), expected);
}