use std::ops::BitXor;
use std::cmp;
use std::iter;
use num_rational::Ratio;


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

    for item in key.iter().take(remainder) {
        vec.push(*item);
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

pub fn edit_distance(str1: &[u8], str2: &[u8]) -> Option<usize> {
    let hmetric = |x, y| { 
        let mut val  = x ^ y;
        let mut dist = 0;

        while val != 0 {
            dist += 1;
            val  &= val - 1;
        }

        dist
    };

    if str1.len() != str2.len() {
        return None;
    }

    let mut dist = 0;
    for (byte1, byte2) in str1.into_iter().zip(str2) {
        dist += hmetric(byte1, byte2);
    }

    Some(dist)
}

pub fn normalized_edit_distance(str1: &[u8], str2: &[u8]) -> Option<Ratio<usize>> {
    let dist = edit_distance(str1, str2);
    let normalized_edit_distance = match dist {
        None       => None,
        Some(edit_dist) => Some(Ratio::new_raw(edit_dist, str1.len())),
    };

    normalized_edit_distance
}

fn total_edit_distance(seq1: &[&[u8]], seq2: &[&[u8]]) -> Option<usize> {
    if seq1.is_empty() || seq2.is_empty() {
        return None;
    }

    if seq1.len() != seq2.len() {
        return None;
    }

    for (str1, str2) in seq1.into_iter().zip(seq2) {
        if str1.len() != str2.len() {
            return None;
        }
    }

    let edit_dist = seq1.into_iter().zip(seq2).fold(0, |dist, (str1, str2)| { 
        dist + edit_distance(str1, str2).unwrap()
    });

    Some(edit_dist)
}

pub fn mean_edit_distance(strings: &[&[u8]]) -> Option<Ratio<usize>> {
    let length = |strings: &[&[u8]]| { 
        strings.into_iter()
               .fold(0, |acc, st| { acc + st.len() }) 
    };

    let seq1: Vec<&[u8]> = strings.into_iter()
                                  .take(strings.len()-1)
                                  .map(|slice| { slice.clone() })
                                  .collect();

    let seq2: Vec<&[u8]> = strings.into_iter()
                                  .skip(1)
                                  .map(|slice| { slice.clone() })
                                  .collect();
                                  
    let dist  = total_edit_distance(seq1.as_ref(), seq2.as_ref());
    let total = length(strings);

    let mean = match dist {
        None      => None,
        Some(val) => Some(Ratio::new_raw(val, total)),
    };

    mean
}
