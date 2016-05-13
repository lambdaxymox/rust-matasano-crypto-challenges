use std::collections::hash_map::HashMap;


fn compute_letter_counts(corpus: &[u8]) -> HashMap<u8, usize> {
    let mut table = HashMap::<u8, usize>::new();

    for character in corpus {
        if table.contains_key(&character) {
            *table.get_mut(&character).unwrap() += 1;
        } else {
            table.insert(*character, 1);
        }
    }

    table
}

fn histogram_size(counts: &HashMap<u8, usize>) -> usize {
    counts.values().fold(0, |acc: usize, count: &usize| { acc + *count })
}

fn to_histogram(counts: &HashMap<u8, usize>) -> HashMap<u8, f64> {
    let total = histogram_size(counts);

    counts.into_iter()
          .map(|(key, value): (&u8, &usize)| {(*key, (*value as f64)/(total as f64))})
          .collect()
}

fn relative_freqs(corpus: &[u8]) -> HashMap<u8, f64> {
    to_histogram(&compute_letter_counts(corpus))
}
/*
fn transpose_histograms(reference: &HashMap<u8, f64>, freqs: &HashMap<u8, f64>) -> HashMap<u8, f64> {
    let map    = HashMap::new();
    let keys   = reference.keys().collect::<Vec<&u8>>().as_slice().sort();
    /*
    let values = freqs.values().collect::<Vec<f64>>().as_slice().sort();

    for i in 0..keys.len() {
        map.insert(keys[i], values[i])
    }
*/
    map
}
*/
