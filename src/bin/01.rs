advent_of_code::solution!(1);

use itertools::Itertools;

fn parse_list(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            if let Some((x, y)) = line
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect_tuple()
            {
                (x, y)
            } else {
                panic!("Invalid input")
            }
        })
        .unzip();

    left.sort();
    right.sort();

    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (left, right) = parse_list(input);

    Some(
        left.iter()
            .zip(right.iter())
            .map(|(l, r)| if l > r { l - r } else { r - l })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_list(input);

    left.iter()
        .map(|l| {
            let match_count = right.iter().filter(|&r| l == r).count();

            l * match_count as u32
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
