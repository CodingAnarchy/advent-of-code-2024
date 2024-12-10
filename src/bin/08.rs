advent_of_code::solution!(8);

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> (i32, i32, HashMap<char, Vec<(i32, i32)>>) {
    let mut max_h = 0;
    let mut max_w = 0;
    let mut locs = HashMap::<char, Vec<(i32, i32)>>::new();

    input.lines().enumerate().for_each(|(y, line)| {
        max_h = max_h.max(y as i32 + 1);
        max_w = max_w.max(line.len() as i32);

        line.chars()
            .enumerate()
            .filter(|(_i, c)| *c != '.')
            .for_each(|(x, c)| {
                locs.entry(c)
                    .or_insert(Vec::new())
                    .push((x as i32, y as i32));
            })
    });

    (max_w, max_h, locs)
}

fn calculate_ans<I>(map_w: i32, map_h: i32, locs: &HashMap<char, Vec<(i32, i32)>>, it: I) -> usize
where
    I: Iterator<Item = i32> + Clone,
{
    let mut antinodes = HashSet::new();

    let mut find_antinodes = |a: (i32, i32), b: (i32, i32)| {
        for (p1, p2) in [(a, b), (b, a)] {
            for i in it.clone() {
                let x = p1.0 + i * (p1.0 - p2.0);
                let y = p1.1 + i * (p1.1 - p2.1);
                if x < 0 || y < 0 || x >= map_w || y >= map_h {
                    break;
                } else {
                    antinodes.insert((x, y));
                }
            }
        }
    };

    locs.iter().for_each(|(_, vs)| {
        vs.iter()
            .combinations(2)
            .for_each(|v| find_antinodes(*v[0], *v[1]))
    });

    antinodes.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (max_w, max_h, locs) = parse_input(input);
    let antinode_count = calculate_ans(max_w, max_h, &locs, std::iter::once(1i32));

    Some(antinode_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (max_w, max_h, locs) = parse_input(input);
    let antinode_count = calculate_ans(max_w, max_h, &locs, 0i32..);

    Some(antinode_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
