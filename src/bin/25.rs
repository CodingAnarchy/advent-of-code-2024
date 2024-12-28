advent_of_code::solution!(25);

use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
struct Schematic {
    is_key: bool,
    heights: [u8; 5],
}

impl Schematic {
    fn fits(&self, other: &Schematic) -> bool {
        if !(self.is_key ^ other.is_key) {
            return false;
        }

        self.heights
            .iter()
            .zip(other.heights.iter())
            .all(|(a, b)| a + b <= 7)
    }
}

impl FromStr for Schematic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().map(|l| l.chars().collect_vec()).collect_vec();
        let is_key = lines[0][0] == '.';

        if !is_key {
            lines.reverse();
        }

        let heights = (0..lines[0].len())
            .map(|col| lines.iter().map(|l| l[col]).filter(|&c| c == '#').count() as u8)
            .collect_vec();

        Ok(Self {
            is_key,
            heights: heights[..5].try_into().unwrap(),
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematics = input
        .split("\n\n")
        .map(|s| Schematic::from_str(s).unwrap())
        .collect_vec();

    let (keys, locks): (Vec<_>, Vec<_>) = schematics.iter().partition(|s| s.is_key);

    keys.iter()
        .cartesian_product(locks)
        .filter(|(key, lock)| key.fits(lock))
        .count()
        .try_into()
        .ok()
}

// pub fn part_two(input: &str) -> Option<u32> {
//     None
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
