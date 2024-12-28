advent_of_code::solution!(24);

use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpType {
    And,
    Or,
    Xor,
}

struct Op {
    inputs: Vec<String>,
    output: String,
    op: OpType,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut wires = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (wire, val) = line.split_once(": ").unwrap();
            (wire.to_string(), val.parse().unwrap())
        })
        .collect::<HashMap<_, _>>();

    let mut ops = input
        .lines()
        .skip_while(|line| !line.contains("->"))
        .map(|line| {
            let (inputs, output) = line.split_once(" -> ").unwrap();
            let (input_a, op, input_b) = inputs
                .split(' ')
                .map(|s| s.to_string())
                .collect_tuple()
                .unwrap();
            let inputs = vec![input_a, input_b];
            let output = output.to_string();
            let op = match op.as_str() {
                "AND" => OpType::And,
                "OR" => OpType::Or,
                "XOR" => OpType::Xor,
                _ => panic!("Unknown op: {}", op),
            };
            Op { inputs, output, op }
        })
        .collect::<VecDeque<_>>();

    while let Some(op) = ops.pop_front() {
        let inputs = op
            .inputs
            .iter()
            .map(|input| wires.get(input))
            .collect_tuple();

        if let Some((Some(a), Some(b))) = inputs {
            let result = match op.op {
                OpType::And => a & b,
                OpType::Or => a | b,
                OpType::Xor => a ^ b,
            };
            wires.insert(op.output, result);
        } else {
            ops.push_back(op);
        }
    }

    let mut end_byte = 0;
    let mut result = vec!['0'; 64];

    wires
        .iter()
        .filter(|(w, _)| w.starts_with("z"))
        .for_each(|(wire, val)| {
            let ord = wire[1..].parse().unwrap();
            result[ord] = char::from_digit(*val, 2).unwrap();
            if ord > end_byte {
                end_byte = ord;
            }
        });

    let output = usize::from_str_radix(&result[..=end_byte].iter().rev().join(""), 2).unwrap();
    Some(output)
}

pub fn part_two(input: &str) -> Option<String> {
    let wires = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (wire, val) = line.split_once(": ").unwrap();
            (wire.to_string(), val.parse::<u8>().unwrap())
        })
        .collect::<HashMap<_, _>>();

    let ops = input
        .lines()
        .skip_while(|line| !line.contains("->"))
        .map(|line| {
            let (inputs, output) = line.split_once(" -> ").unwrap();
            let (input_a, op, input_b) = inputs
                .split(' ')
                .map(|s| s.to_string())
                .collect_tuple()
                .unwrap();
            let inputs = vec![input_a, input_b];
            let output = output.to_string();
            let op = match op.as_str() {
                "AND" => OpType::And,
                "OR" => OpType::Or,
                "XOR" => OpType::Xor,
                _ => panic!("Unknown op: {}", op),
            };
            Op { inputs, output, op }
        })
        .collect::<Vec<_>>();

    let bitlen = wires.len() as u8 / 2;

    let mut swapped = HashSet::<String>::new();
    let z00 = ops.iter().find(|op| {
        op.inputs.iter().sorted().collect::<Vec<&String>>() == ["x00", "y00"]
            && op.op == OpType::Xor
    });

    if let Some(z00) = z00 {
        if z00.output != "z00" {
            swapped.insert(z00.output.clone());
        }
    }

    let mut carry: String = ops
        .iter()
        .find_map(|op| {
            if op.inputs.iter().sorted().collect::<Vec<&String>>() == ["x00", "y00"]
                && op.op == OpType::And
            {
                Some(&op.output)
            } else {
                None
            }
        })
        .unwrap()
        .to_string();

    for bit in 1..bitlen {
        let x = format!("x{:02}", bit);
        let y = format!("y{:02}", bit);
        let z = format!("z{:02}", bit);

        let basic_add = ops
            .iter()
            .find(|op| {
                op.inputs.iter().sorted().collect::<Vec<&String>>() == [&x, &y]
                    && op.op == OpType::Xor
            })
            .unwrap()
            .output
            .clone();

        let add = ops
            .iter()
            .find(|op| {
                op.op == OpType::Xor
                    && (op.inputs.contains(&basic_add) || op.inputs.contains(&carry))
            })
            .unwrap();

        if add.output != z {
            swapped.insert(z);
            swapped.insert(add.output.clone());
        }

        if !add.inputs.contains(&basic_add) {
            swapped.insert(basic_add.clone());
        }

        if !add.inputs.contains(&carry) {
            swapped.insert(carry.clone());
        }

        // check basic carry - only output can be wrong
        let basic_carry = ops
            .iter()
            .find(|op| {
                op.inputs.iter().sorted().collect::<Vec<&String>>() == [&x, &y]
                    && op.op == OpType::And
            })
            .unwrap()
            .output
            .clone();
        let cascade_carry = ops
            .iter()
            .find(|op| {
                op.op == OpType::And
                    && (op.inputs.contains(&basic_add) || op.inputs.contains(&carry))
            })
            .unwrap();

        if !cascade_carry.inputs.contains(&basic_add) {
            swapped.insert(basic_add.to_string());
        }

        if !cascade_carry.inputs.contains(&carry) {
            swapped.insert(carry.to_string());
        }

        let carry_gate = ops
            .iter()
            .find(|op| {
                op.op == OpType::Or
                    && (op.inputs.contains(&basic_carry)
                        || op.inputs.contains(&cascade_carry.output))
            })
            .unwrap();

        if !carry_gate.inputs.contains(&cascade_carry.output) {
            swapped.insert(cascade_carry.output.to_string());
        }

        if !carry_gate.inputs.contains(&basic_carry) {
            swapped.insert(basic_carry.to_string());
        }

        carry = carry_gate.output.clone();
    }

    let swapped = swapped.into_iter().sorted().collect::<Vec<String>>();
    Some(swapped.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file_part(
    //         "examples", DAY, 2,
    //     ));
    //     assert_eq!(result, None);
    // }
}
