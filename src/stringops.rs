use std::iter;

pub fn repeat_char(character: u8, n: usize) -> Vec<u8> {
    iter::repeat(character).take(n).collect()
}