advent_of_code::solution!(11);

use std::collections::HashMap;

#[derive(Default, Debug)]
struct StoneConfiguration {
    zero_count: usize,
    even_digits: HashMap<usize, usize>,
    odd_digits: HashMap<usize, usize>,
}

impl StoneConfiguration {
    fn insert_number(&mut self, number: usize, count: usize) {
        if number == 0 {
            self.zero_count += count;
            return;
        }

        let digits = count_digits(number);
        match digits % 2 {
            0 => {
                let entry = self.even_digits.entry(number).or_insert(0);
                *entry += count;
            }
            _ => {
                let entry = self.odd_digits.entry(number).or_insert(0);
                *entry += count;
            }
        }
    }

    fn blink(&self) -> StoneConfiguration {
        let mut new_config = StoneConfiguration::default();

        // Convert 0s to 1s
        new_config.insert_number(1, self.zero_count);

        // Split even digits
        for (number, count) in self.even_digits.iter() {
            let (left, right) = split_number(*number);
            new_config.insert_number(left, *count);
            new_config.insert_number(right, *count);
        }

        // Multiply odd digits by 2024
        for (number, count) in self.odd_digits.iter() {
            new_config.insert_number(number * 2024, *count);
        }

        new_config
    }

    fn size(&self) -> usize {
        &self.zero_count
            + &self.even_digits.values().sum::<usize>()
            + &self.odd_digits.values().sum::<usize>()
    }
}

fn count_digits(mut number: usize) -> usize {
    let mut count = 0;
    while number > 0 {
        count += 1;
        number /= 10;
    }
    count
}

fn split_number(number: usize) -> (usize, usize) {
    let digit_count = count_digits(number);

    let mut left = number;
    let mut right = 0;
    let mut multiplier = 1;

    for _ in 0..digit_count / 2 {
        right += (left % 10) * multiplier;
        multiplier *= 10;
        left /= 10;
    }

    (left, right)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut stones = StoneConfiguration::default();

    for number in input.split_whitespace().map(|s| s.parse::<usize>()) {
        if let Ok(number) = number {
            stones.insert_number(number, 1);
        }
    }

    for _ in 0..25 {
        stones = stones.blink();
    }

    Some(stones.size())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut stones = StoneConfiguration::default();

    for number in input.split_whitespace().map(|s| s.parse::<usize>()) {
        if let Ok(number) = number {
            stones.insert_number(number, 1);
        }
    }

    for _ in 0..75 {
        stones = stones.blink();
    }

    Some(stones.size())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482)); //inferred from running the code; not provided in
                                                  //the examples
    }
}
