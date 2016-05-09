use std::vec::Vec;


pub trait ToHexRep {
    type Target; 

    fn to_hex(&self)       -> Option<Vec<u8>>;
    fn validate_hex(&self) -> bool; 
}

fn to_hex_digit(character: u8) -> Option<u8> {
    match character {
        0x30...0x39 => Some(character-0x30),
        0x41...0x46 => Some(character-0x37),
        0x61...0x66 => Some(character-0x57),
        _           => None,
    }
}

impl ToHexRep for String {
    type Target = String;

    fn to_hex(self: &String) -> Option<Vec<u8>> {
        let mut vec = Vec::new();

        if self.len() <= 0 {
            return Some(vec);
        }

        if !self.validate_hex() {
            return None;
        }

        let slice: &[u8] = self.as_ref();

        if slice.len() % 2 == 0 {
            for i in (0..slice.len()-1).filter(|&i| i % 2 == 0) {
                let upper_nibble = to_hex_digit(slice[i]).unwrap() << 4;
                let lower_nibble = to_hex_digit(slice[i+1]).unwrap();

                vec.push(upper_nibble | lower_nibble);
            }

        } else {
            let top_digit = to_hex_digit(slice[0]).unwrap();
            vec.push(top_digit);

            for i in (1..slice.len()-1).filter(|&i| i % 2 == 0) {
                let upper_nibble = to_hex_digit(slice[i]).unwrap() << 4;
                let lower_nibble = to_hex_digit(slice[i+1]).unwrap();

                vec.push(upper_nibble | lower_nibble);
            }
        }

        Some(vec)
    }

    fn validate_hex(&self) -> bool {
        let slice: &[u8] = self.as_ref();

        for character in slice {
            match *character {
                0x30...0x39 => continue,
                0x41...0x46 => continue,
                0x61...0x66 => continue,
                _           => return false,
            }
        }

        true
    }
}
