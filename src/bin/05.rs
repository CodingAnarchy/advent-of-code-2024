advent_of_code::solution!(5);

#[derive(Debug)]
struct Rule {
    first: u32,
    second: u32,
}

impl Rule {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split('|');
        let first = parts.next().unwrap().parse().unwrap();
        let second = parts.next().unwrap().parse().unwrap();

        Self { first, second }
    }

    fn is_valid(&self, update: &Vec<u32>) -> bool {
        let first_pos = update.iter().position(|&x| x == self.first);
        let second_pos = update.iter().position(|&x| x == self.second);

        match (first_pos, second_pos) {
            (Some(first), Some(second)) => first < second,
            _ => true, // doesn't have both values, so the rule is ignored
        }
    }
}

fn reorder(update: &Vec<u32>, rules: &Vec<Rule>) -> Vec<u32> {
    let mut new_update = update.clone();

    // TODO: This is brute force approach to keep looping until it is valid and
    // while it works, we could do better by iterating over the positions in the update
    // and then applying any invalid rules to perform swaps until each position is valid
    while rules.iter().any(|rule| !rule.is_valid(&new_update)) {
        for rule in rules {
            let first_pos = new_update.iter().position(|&x| x == rule.first);
            let second_pos = new_update.iter().position(|&x| x == rule.second);

            match (first_pos, second_pos) {
                (Some(first), Some(second)) => {
                    if first > second {
                        new_update.swap(first, second);
                    }
                }
                _ => {}
            }
        }
    }

    new_update
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rules = Vec::<Rule>::new();
    let mut updates = Vec::<Vec<u32>>::new();

    for line in input.lines() {
        if line.contains('|') {
            rules.push(Rule::from_str(line));
        } else if line.contains(',') {
            updates.push(line.split(',').map(|x| x.parse().unwrap()).collect());
        }
    }

    updates
        .iter()
        .filter(|&update| rules.iter().all(|rule| rule.is_valid(update)))
        .map(|update| update[update.len() / 2])
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rules = Vec::<Rule>::new();
    let mut updates = Vec::<Vec<u32>>::new();

    for line in input.lines() {
        if line.contains('|') {
            rules.push(Rule::from_str(line));
        } else if line.contains(',') {
            updates.push(line.split(',').map(|x| x.parse().unwrap()).collect());
        }
    }

    updates
        .iter()
        .filter(|&update| rules.iter().any(|rule| !rule.is_valid(update)))
        .map(|update| reorder(&update, &rules))
        .map(|update| update[update.len() / 2])
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
