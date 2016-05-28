use super::base64;
use super::hex_rep::ToHexRep;
use super::bitwiseops;

#[test]
fn test_to_base64() {
    let hex_str  = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let result   = base64::to_base64(&String::from_utf8(hex_str.to_hex().unwrap()).unwrap());
    let expected = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

    assert_eq!(expected.len(), result.len());
    assert_eq!(expected, result);
}

#[test]
fn test_to_hex() {
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
fn test_edit_dist() {
    let str1 = "this is a test";
    let str2 = "wokka wokka!!!";
    let dist = bitwiseops::edit_distance(str1.as_ref(), str2.as_ref()).unwrap();

    assert_eq!(dist, 37);
}

#[test]
fn test_edit_dist_should_reject_mismatched_length_strings() {
    let str1 = "this is a test";
    let str2 = "this is a malformed string";
    let dist = bitwiseops::edit_distance(str1.as_ref(), str2.as_ref());

    assert!(dist.is_none());
}