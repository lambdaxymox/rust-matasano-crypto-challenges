use std::ops::BitXor;
use std::cmp;
use std::iter;


pub fn block_xor<T>(block1: &[T], block2: &[T]) -> Vec<T::Output> 
    where T: BitXor + Clone + Copy {

    let block_size = cmp::min(block1.len(), block2.len());

    let mut new_block = Vec::with_capacity(block_size);

    for i in 0..block_size {
        new_block.push(block1[i] ^ block2[i]);
    }

    new_block
}

pub fn exact_block_xor<T>(block1: &[T], block2: &[T]) -> Option<Vec<T::Output>>
    where T: BitXor + Clone + Copy {

    if block1.len() != block2.len() {
        return None;
    }

    Some(block_xor(block1, block2))
}

fn repeat_char(character: u8, n: usize) -> Vec<u8> {
    iter::repeat(character).take(n).collect()
}

fn op_with_char<F>(op: F, character: u8, string: &[u8]) -> Vec<u8>
    where F: Fn(&[u8], &[u8]) -> Vec<u8> {

    let other_string = repeat_char(character, string.len());

    op(string, &other_string)
}

pub fn xor_with_char(character: u8, string: &[u8]) -> Vec<u8> {
    op_with_char(&block_xor, character, string)
}

#[allow(unused_variables)]
fn with_key(key: &[u8], string: &[u8]) -> Vec<u8> {
    let rep_count = string.len() / key.len();
    let remainder = string.len() % key.len();
    let mut vec   = Vec::new();

    assert!(string.len() == rep_count*key.len() + remainder);

    for i in 0..rep_count {
        for ch in key {
            vec.push(*ch);
        }
    }

    for i in 0..remainder {
        vec.push(key[i]);
    }    

    vec
}

fn op_with_key<F>(op: F, key: &[u8], string: &[u8]) -> Vec<u8>
    where F: Fn(&[u8], &[u8]) -> Vec<u8> {

    let other_string = with_key(key, string);

    op(string, other_string.as_ref())
}

pub fn xor_with_key(key: &[u8], string: &[u8]) -> Vec<u8> {
    op_with_key(&block_xor, key, string)
}
