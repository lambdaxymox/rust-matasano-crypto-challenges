use cryptanalysis::frequency_analysis::Histogram;
use num_rational::Ratio;
use std::iter::IntoIterator;
use std::hash::Hash;
use bitwise::bitwiseops::xor_with_char;


fn score_func<T>(model: &Histogram<T>, freqs: &Histogram<T>) -> Ratio<usize>
    where T: Eq + Hash {

    freqs.into_iter().fold(Ratio::from_integer(0), |score, (key, _)| { 
        if model.contains_key(key) {
            score + model.get(key).unwrap()
        } else {
            score
        }
    })
}

fn score_with<T>(model: &Histogram<T>, string: &[T]) -> Ratio<usize>
    where T: Eq + Hash + Copy {

    score_func(model, &Histogram::from(string))
}

fn max_char_with(model: &Histogram<u8>, charset: &[u8], string: &[u8]) -> (u8, Ratio<usize>) {
    let mut max_score = Ratio::from_integer(0);
    let mut max_char  = 0x00;

    for ch in charset {
        let cipher_text = xor_with_char(*ch, string);
        let score = score_with(model, &cipher_text);

        if score > max_score {
            max_score = score;
            max_char  = *ch;
        }
    }

    (max_char, max_score)
}

fn min_char_with(model: &Histogram<u8>, charset: &[u8], string: &[u8]) -> (u8, Ratio<usize>) {
    let mut min_score = Ratio::from_integer(0);
    let mut min_char  = 0x00;

    for ch in charset {
        let cipher_text = xor_with_char(*ch, string);
        let score = score_with(model, &cipher_text);

        if score < min_score {
            min_score = score;
            min_char  = *ch;
        }
    }

    (min_char, min_score)
}

pub fn break_xor_char(model: &Histogram<u8>, charset: &[u8], string: &[u8]) -> (u8, Ratio<usize>) {
    max_char_with(model, charset, string)
}
