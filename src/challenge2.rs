use super::bitwiseops;

fn challenge2() {
    let hex1  = String::from("1c0111001f010100061a024b53535009181c").to_hex().unwrap();
    let hex2  = String::from("686974207468652062756c6c277320657965").to_hex().unwrap();
    let result   = bitwiseops::exact_block_xor(hex1.as_slice(), hex2.as_slice()).unwrap();
    let expected = String::from("746865206b696420646f6e277420706c6179").to_hex().unwrap();

    assert_eq!(expected, result);
}