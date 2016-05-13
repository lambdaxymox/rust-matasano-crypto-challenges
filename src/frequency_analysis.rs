use std::collections::hash_map::HashMap;
use num_rational::Ratio;


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

fn to_histogram(counts: &HashMap<u8, usize>) -> HashMap<u8, Ratio<usize>> {
    let total = histogram_size(counts);

    counts.into_iter()
          .map(|(key, value): (&u8, &usize)| {(*key, Ratio::new_raw(*value, total))})
          .collect()
}

fn relative_freqs(corpus: &[u8]) -> HashMap<u8, Ratio<usize>> {
    to_histogram(&compute_letter_counts(corpus))
}

fn transpose_histograms(reference: &HashMap<u8, Ratio<usize>>, freqs: &HashMap<u8, Ratio<usize>>) -> HashMap<u8, Ratio<usize>> {
    let mut map      = HashMap::new();
    
    let mut keys = Vec::new();
    for key in reference.keys() {
        keys.push(key.clone());
    }
    keys.sort();

    let mut values = Vec::new();
    for value in freqs.values() {
        values.push(value.clone());
    }
    values.sort();


    for i in 0..keys.len() {
        map.insert(keys[i], values[i]);
    }

    map
}
