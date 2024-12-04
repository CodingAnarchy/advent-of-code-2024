advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    mul_regex
        .captures_iter(input)
        .map(|c| c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap())
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)|(don't\(\))|(do\(\))").unwrap();

    let mut execute_mul = true;

    mul_regex
        .captures_iter(input)
        .map(|c| {
            match &c[0] {
                "don't()" => execute_mul = false,
                "do()" => execute_mul = true,
                _ => (),
            }

            // Return early if the numbers didn't get extracted, this is not a mul command
            if let None = c.get(1) {
                return 0;
            }

            if execute_mul {
                c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap()
            } else {
                0
            }
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
