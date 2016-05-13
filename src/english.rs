use std::collections::hash_map::HashMap;

const raw_frequency_table: HashMap<u8, f64> = 
    HashMap::new()
            .insert('E' as u8, 0.12702).insert('T' as u8, 0.09056).insert('E' as u8, 0.12702)
            .insert('T' as u8, 0.09056).insert('A' as u8, 0.08167).insert('O' as u8, 0.07507)
            .insert('I' as u8, 0.06966).insert('N' as u8, 0.06749).insert('S' as u8, 0.06327)
            .insert('H' as u8, 0.06094).insert('R' as u8, 0.05987).insert('D' as u8, 0.04253)
            .insert('L' as u8, 0.04025).insert('C' as u8, 0.02782).insert('U' as u8, 0.02758)
            .insert('M' as u8, 0.02406).insert('W' as u8, 0.02361).insert('F' as u8, 0.02228)
            .insert('G' as u8, 0.02015).insert('Y' as u8, 0.01974).insert('P' as u8, 0.01929)
            .insert('B' as u8, 0.01492).insert('V' as u8, 0.00978).insert('K' as u8, 0.00772)
            .insert('J' as u8, 0.00153).insert('X' as u8, 0.00150).insert('Q' as u8, 0.00095)
            .insert('Z' as u8, 0.00074);

fn score_func() -> f64 {
    0.0
}