advent_of_code::solution!(7);

fn check_permutation(left: u64, right: &[u64], target: u64, concatenate: bool) -> bool {
    if right.is_empty() {
        return left == target;
    } else if left > target {
        return false;
    } else {
        let current = right[0];
        check_permutation(left + current, &right[1..], target, concatenate)
            || check_permutation(left * current, &right[1..], target, concatenate)
            || (concatenate
                && check_permutation(
                    left * 10u64.pow(1 + current.ilog10()) + current,
                    &right[1..],
                    target,
                    concatenate,
                ))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .filter_map(|line| {
            let (output, params) = line.split_once(":").unwrap();
            let val = output.trim().parse::<u64>().unwrap();
            let params = params
                .trim()
                .split(" ")
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            if check_permutation(params[0], &params[1..], val, false) {
                Some(val)
            } else {
                None
            }
        })
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    input
        .lines()
        .filter_map(|line| {
            let (output, params) = line.split_once(":").unwrap();
            let val = output.trim().parse::<u64>().unwrap();
            let params = params
                .trim()
                .split(" ")
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            if check_permutation(params[0], &params[1..], val, true) {
                Some(val)
            } else {
                None
            }
        })
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
