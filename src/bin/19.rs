advent_of_code::solution!(19);

use std::collections::{HashMap, HashSet};

fn combinations<'a>(
    start: &'a str,
    towels: &HashSet<&str>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if memo.contains_key(start) {
        return memo[start];
    }

    for i in 1..=start.len().min(8) {
        let prefix = &start[..i];
        let postfix = &start[i..];
        if towels.contains(prefix) {
            let combs = combinations(postfix, towels, memo);
            memo.entry(start)
                .and_modify(|e| *e += combs)
                .or_insert(combs);
        }
    }

    *memo.get(start).unwrap_or(&0)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (avail, rem) = input.split_once("\n\n").unwrap();

    let towels = avail.split(", ").collect::<HashSet<_>>();
    let mut memo = HashMap::new();
    memo.insert("", 1);
    let res = rem
        .lines()
        .filter(|l| combinations(l, &towels, &mut memo) > 0)
        .count();

    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (avail, rem) = input.split_once("\n\n").unwrap();

    let towels = avail.split(", ").collect::<HashSet<_>>();
    let mut memo = HashMap::new();
    memo.insert("", 1);
    let res = rem
        .lines()
        .filter_map(|l| {
            let combs = combinations(l, &towels, &mut memo);
            match combs {
                0 => None,
                _ => Some(combs as usize),
            }
        })
        .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
