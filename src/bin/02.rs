advent_of_code::solution!(2);

fn is_report_safe(report: &[u32]) -> bool {
    if report.is_sorted() {
        if report.windows(2).all(|w| {
            let window_diff = w[1] - w[0];
            (1..=3).contains(&window_diff)
        }) {
            return true;
        }
    } else if report.is_sorted_by(|a, b| b <= a)
        && report.windows(2).all(|w| {
            let window_diff = w[0] - w[1];
            (1..=3).contains(&window_diff)
        })
    {
        return true;
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let safe_reports = input
        .lines()
        .filter_map(|line| {
            let report: Vec<u32> = line
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            if is_report_safe(&report) {
                return Some(true);
            }

            None
        })
        .count();

    Some(safe_reports as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_reports = input
        .lines()
        .filter_map(|line| {
            let report: Vec<u32> = line
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            if is_report_safe(&report) {
                return Some(true);
            }

            let report_copy = report.clone();
            for (idx, _n) in report.iter().enumerate() {
                let index = idx;
                let mut dampened_report = report_copy[0..index].to_vec();
                dampened_report.append(&mut report_copy[index + 1..report_copy.len()].to_vec());
                if is_report_safe(&dampened_report) {
                    return Some(true);
                }
            }

            None
        })
        .count();

    Some(safe_reports as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
