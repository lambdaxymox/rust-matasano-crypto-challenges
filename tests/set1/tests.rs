use matasano_crypto_challenges::bitwise::base64;
use matasano_crypto_challenges::bitwise::hex_rep::ToHexRep;
use matasano_crypto_challenges::set1::challenge1::CHALLENGE1;
use matasano_crypto_challenges::set1::challenge2::CHALLENGE2;
use matasano_crypto_challenges::set1::challenge3::CHALLENGE3;
use matasano_crypto_challenges::set1::challenge4::CHALLENGE4;
use matasano_crypto_challenges::set1::challenge5::CHALLENGE5;
use matasano_crypto_challenges::set1::challenge6::CHALLENGE6;


#[test]
fn test_challenge1() {
    let hex_str  = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let expected   = base64::to_base64(&String::from_utf8(hex_str.to_hex().unwrap()).unwrap());

    assert_eq!(CHALLENGE1.execute(), expected);
}

#[test]
fn test_challenge2() {
    let expected = String::from_utf8(String::from("746865206b696420646f6e277420706c6179")
                          .to_hex().unwrap())
                          .unwrap();

    assert_eq!(CHALLENGE2.execute(), expected);
}

#[test]
fn test_challenge3() {
    let expected = String::from_utf8(Vec::from("Cooking MC's like a pound of bacon")).unwrap();

    assert_eq!(CHALLENGE3.execute(), expected);
}

#[test]
fn test_challenge4() {
    let expected = "Now that the party is jumping\n";

    assert_eq!(CHALLENGE4.execute(), expected);
}

#[test]
fn test_challenge5() {
    let expected = String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f")
                          .to_hex().unwrap();
    let result = CHALLENGE5.execute();

    assert_eq!(result.as_bytes(), expected.as_slice());
}

#[test]
fn test_challenge6() {
    let result = CHALLENGE6.execute();
    let expected = "T+rminator X: Bring the noise";

    assert_eq!(result, expected);
}