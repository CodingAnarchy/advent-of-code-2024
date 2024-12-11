advent_of_code::solution!(9);

use itertools::Itertools;
use std::iter::{once, repeat_n};

pub fn part_one(input: &str) -> Option<usize> {
    let chars = input.lines().flat_map(|line| line.chars());

    let mut memory: Vec<Option<usize>> = chars
        .chain(once('0'))
        .tuples()
        .enumerate()
        .flat_map(|(i, (file, free))| {
            repeat_n(Some(i), file.to_digit(10).unwrap() as usize)
                .chain(repeat_n(None, free.to_digit(10).unwrap() as usize))
        })
        .collect();

    let mut head = 0;
    let mut tail = memory.len() - 1;

    while head != tail {
        if memory[head].is_some() {
            head += 1;
            continue;
        } else if memory[tail].is_none() {
            tail -= 1;
            continue;
        }

        memory[head] = memory[tail];
        head += 1;
        tail -= 1;
    }

    Some(
        memory[0..=head]
            .iter()
            .copied()
            .while_some()
            .enumerate()
            .map(|(i, x)| x * i)
            .sum(),
    )
}

#[derive(Debug, Clone, Copy)]
struct File {
    id: usize,
    size: usize,
}

#[derive(Debug, Clone, Copy)]
enum Data {
    Used(File),
    Free(usize),
}

pub fn part_two(input: &str) -> Option<usize> {
    let chars = input.lines().flat_map(|line| line.chars());

    let mut fstab = chars
        .chain(once('0'))
        .map(|c| c.to_digit(10).unwrap() as usize)
        .tuples()
        .enumerate()
        .flat_map(|(i, (file, free))| [Data::Used(File { id: i, size: file }), Data::Free(free)])
        .collect::<Vec<Data>>();

    let files: Vec<File> = fstab
        .iter()
        .rev()
        .filter_map(|data| match data {
            Data::Used(file) => Some(*file),
            Data::Free(_) => None,
        })
        .collect();

    files.iter().for_each(|file| {
        let new_pos = fstab
            .iter()
            .position(|data| matches!(data, Data::Free(size) if *size >= file.size));

        if let Some(pos) = new_pos {
            let old_pos = fstab
                .iter()
                .position(|data| matches!(data, Data::Used(f) if f.id == file.id))
                .unwrap();

            if pos >= old_pos {
                return;
            }

            fstab.insert(pos, Data::Used(*file));
            if let Data::Free(size) = fstab[pos + 1] {
                if size - file.size == 0 {
                    fstab.remove(pos + 1);
                    fstab[old_pos] = Data::Free(file.size);
                } else {
                    fstab[pos + 1] = Data::Free(size - file.size);
                    fstab[old_pos + 1] = Data::Free(file.size);
                }
            }
        }
    });

    let memory: Vec<Option<usize>> = fstab
        .iter()
        .flat_map(|data| match data {
            Data::Used(file) => repeat_n(Some(file.id), file.size),
            Data::Free(size) => repeat_n(None, *size),
        })
        .collect();

    Some(
        memory
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| match x {
                Some(x) => x * i,
                None => 0,
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
