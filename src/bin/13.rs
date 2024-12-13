advent_of_code::solution!(13);

use itertools::Itertools;
use regex::Regex;

// structure each construct as a system of equations
// X_A + X_B = X_Prize
// Y_A + Y_B = Y_Prize
struct Equations {
    x: [u64; 3],
    y: [u64; 3],
}

impl Equations {
    fn new(a: (u64, u64), b: (u64, u64), prize: (u64, u64)) -> Self {
        let x = [a.0 as u64, b.0 as u64, prize.0 as u64];
        let y = [a.1 as u64, b.1 as u64, prize.1 as u64];

        Self { x, y }
    }

    fn solve(&self) -> Option<u64> {
        let det = (self.x[0] * self.y[1]) as i64 - (self.x[1] * self.y[0]) as i64;
        if det == 0 {
            return None;
        }

        let det_a = (self.x[2] * self.y[1]) as i64 - (self.x[1] * self.y[2]) as i64;
        let det_b = (self.x[0] * self.y[2]) as i64 - (self.x[2] * self.y[0]) as i64;

        if det_a % det != 0 || det_b % det != 0 {
            return None;
        }

        let a = det_a / det;
        let b = det_b / det;

        if a < 0 || b < 0 {
            return None;
        }

        // Multiply by 3 to get the final answer for A because that costs
        // 3 tokens while B costs 1 token
        Some((a * 3 + b) as u64)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let button_regex = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

    let equations = input
        .lines()
        .chunk_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(empty, mut chunk)| {
            if empty {
                return None;
            }

            let a = chunk.next().unwrap();
            let b = chunk.next().unwrap();
            let prize = chunk.next().unwrap();

            let a = button_regex.captures(a).unwrap();
            let b = button_regex.captures(b).unwrap();
            let prize = prize_regex.captures(prize).unwrap();

            Some(Equations::new(
                (a[1].parse().unwrap(), a[2].parse().unwrap()),
                (b[1].parse().unwrap(), b[2].parse().unwrap()),
                (prize[1].parse().unwrap(), prize[2].parse().unwrap()),
            ))
        })
        .collect::<Vec<Equations>>();

    Some(
        equations
            .iter()
            .filter_map(|equation| equation.solve())
            .sum(),
    )
}

const PART_TWO_UNIT_CONVERSION: u64 = 10_000_000_000_000;

pub fn part_two(input: &str) -> Option<u64> {
    let button_regex = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

    let equations = input
        .lines()
        .chunk_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(empty, mut chunk)| {
            if empty {
                return None;
            }

            let a = chunk.next().unwrap();
            let b = chunk.next().unwrap();
            let prize = chunk.next().unwrap();

            let a = button_regex.captures(a).unwrap();
            let b = button_regex.captures(b).unwrap();
            let prize = prize_regex.captures(prize).unwrap();

            Some(Equations::new(
                (a[1].parse().unwrap(), a[2].parse().unwrap()),
                (b[1].parse().unwrap(), b[2].parse().unwrap()),
                (
                    prize[1].parse::<u64>().unwrap() + PART_TWO_UNIT_CONVERSION,
                    prize[2].parse::<u64>().unwrap() + PART_TWO_UNIT_CONVERSION,
                ),
            ))
        })
        .collect::<Vec<Equations>>();

    Some(
        equations
            .iter()
            .filter_map(|equation| equation.solve())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
