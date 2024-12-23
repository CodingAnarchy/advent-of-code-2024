advent_of_code::solution!(22);

use rayon::prelude::*;
use std::collections::HashMap;

fn next_secret_number(mut secret: usize) -> usize {
    secret ^= secret << 6;
    secret %= 16777216; // 2^24
    secret ^= secret >> 5;
    secret %= 16777216; // 2^24
    secret ^= secret << 11;
    secret %= 16777216; // 2^24
    secret
}

fn encode_sequence(d1: i8, d2: i8, d3: i8, d4: i8) -> u32 {
    let result: u32 = (((d1.abs() & 0xf) as u32) << 16)
        | (((d2.abs() & 0xf) as u32) << 12)
        | (((d3.abs() & 0xf) as u32) << 8)
        | (((d4.abs() & 0xf) as u32) << 4);

    let signs = ((if d1 < 0 { 1 } else { 0 }) << 3)
        | ((if d2 < 0 { 1 } else { 0 }) << 2)
        | ((if d3 < 0 { 1 } else { 0 }) << 1)
        | (if d4 < 0 { 1 } else { 0 });

    result | signs
}

pub fn part_one(input: &str) -> Option<usize> {
    let seeds = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let total = seeds
        .par_iter()
        .map(|seed| {
            let mut seed = *seed;
            for _ in 0..2000 {
                seed = next_secret_number(seed);
            }

            seed
        })
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let seeds = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let best_price_sequences = seeds
        .par_iter()
        .map(|seed| {
            let mut seed = *seed;
            let mut last_digit: i8 = (seed % 10) as i8;
            let mut last_sequence = (0i8, 0i8, 0i8, 0i8);
            let mut sequence_best_prices = HashMap::<_, u8>::new();
            for i in 0..2000 {
                seed = next_secret_number(seed);
                let diff = (seed % 10) as i8 - last_digit;
                last_digit = (seed % 10) as i8;
                last_sequence = (last_sequence.1, last_sequence.2, last_sequence.3, diff);
                let sequence_code = encode_sequence(
                    last_sequence.0,
                    last_sequence.1,
                    last_sequence.2,
                    last_sequence.3,
                );

                if i >= 3 {
                    sequence_best_prices
                        .entry(sequence_code)
                        .or_insert(last_digit as u8);
                }
            }

            sequence_best_prices
        })
        .collect::<Vec<_>>();

    let mut best_total = 0;
    let mut sum_map = HashMap::<_, usize>::new();
    best_price_sequences.iter().for_each(|sequence| {
        for (key, &value) in sequence.iter() {
            let old_value = sum_map.entry(key.clone()).or_insert(0);
            *old_value += value as usize;
            if *old_value > best_total {
                best_total = *old_value;
            }
        }
    });

    Some(best_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37_327_623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
