use bitwise::bitwiseops;

trait BlockCipher {
    fn process_block(&self, input: &[u8]) -> Vec<u8>;
}

pub struct SingleCharXorCipher {
    key: u8,
}

impl SingleCharXorCipher {
    fn new(key: u8) -> SingleCharXorCipher {
        SingleCharXorCipher {
            key: key,
        }
    }
}

pub struct XorCipher {
    key: Vec<u8>,
}

impl XorCipher {
    fn new(key: Vec<u8>) -> XorCipher {
        XorCipher {
            key: key.clone(),
        }
    }
}

impl BlockCipher for SingleCharXorCipher {
    fn process_block(&self, input: &[u8]) -> Vec<u8> {
        bitwiseops::xor_with_char(self.key, input)
    }
}

impl BlockCipher for XorCipher {
    fn process_block(&self, input: &[u8]) -> Vec<u8> {
        bitwiseops::xor_with_key(&self.key, input)
    }
}