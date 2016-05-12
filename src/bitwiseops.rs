use std::ops::BitXor;
use std::cmp;


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

