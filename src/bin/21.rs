advent_of_code::solution!(21);

use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const NUMERIC: [[u8; 3]; 4] = [
    [b'7', b'8', b'9'],
    [b'4', b'5', b'6'],
    [b'1', b'2', b'3'],
    [b' ', b'0', b'A'],
];
const DIRECTIONAL: [[u8; 3]; 2] = [[b' ', b'^', b'A'], [b'<', b'v', b'>']];

fn find_shortest_paths(
    keypad: &[[u8; 3]],
    from: u8,
    to: u8,
    cache: &mut HashMap<(u8, u8), Rc<Vec<Vec<u8>>>>,
) -> Rc<Vec<Vec<u8>>> {
    if let Some(cached) = cache.get(&(from, to)) {
        return cached.clone();
    }

    if from == to {
        let result = Rc::new(vec![vec![b'A']]);
        cache.insert((from, to), result.clone());
        return result;
    }

    // find 'from' and 'to' on the keypad
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, row) in keypad.iter().enumerate() {
        for (x, &key) in row.iter().enumerate() {
            if key == from {
                start = (x, y);
            }
            if key == to {
                end = (x, y);
            }
        }
    }

    // flood fill keypad to find shortest path
    let mut dists = vec![vec![usize::MAX; 3]; keypad.len()];
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((pos, dist)) = queue.pop_front() {
        dists[pos.1][pos.0] = dist;
        for (dx, dy) in DIRS {
            let nx = pos.0 as i32 + dx;
            let ny = pos.1 as i32 + dy;
            if nx >= 0
                && ny >= 0
                && nx < 3
                && ny < keypad.len() as i32
                && keypad[ny as usize][nx as usize] != b' '
                && dists[ny as usize][nx as usize] == usize::MAX
            {
                queue.push_back(((nx as usize, ny as usize), dist + 1));
            }
        }
    }

    // backtrace and collect all paths
    let mut paths = vec![];
    let mut stack = vec![(end, vec![b'A'])];
    while let Some((pos, path)) = stack.pop() {
        if pos == start {
            paths.push(path);
            continue;
        }

        for (i, (dx, dy)) in DIRS.iter().enumerate() {
            let nx = pos.0 as i32 + dx;
            let ny = pos.1 as i32 + dy;
            if nx >= 0
                && ny >= 0
                && nx < 3
                && ny < keypad.len() as i32
                && dists[ny as usize][nx as usize] < dists[pos.1][pos.0]
            {
                let c = match i {
                    0 => b'<',
                    1 => b'^',
                    2 => b'>',
                    3 => b'v',
                    _ => unreachable!(),
                };
                let mut new_path = vec![c];
                new_path.extend(&path);
                stack.push(((nx as usize, ny as usize), new_path));
            }
        }
    }

    let result = Rc::new(paths);
    cache.insert((from, to), result.clone());
    result
}

fn find_shortest_sequence(
    s: &[u8],
    depth: usize,
    highest: bool,
    cursors: &mut Vec<u8>,
    cache: &mut HashMap<(Vec<u8>, usize, u8), usize>,
    path_cache: &mut HashMap<(u8, u8), Rc<Vec<Vec<u8>>>>,
) -> usize {
    let cache_key = (s.to_vec(), depth, cursors[depth]);
    if let Some(cached) = cache.get(&cache_key) {
        return *cached;
    }

    let mut result = 0;
    s.iter().for_each(|c| {
        let paths = find_shortest_paths(
            if highest { &NUMERIC } else { &DIRECTIONAL },
            cursors[depth],
            *c,
            path_cache,
        );
        if depth == 0 {
            // all paths are the same length
            result += paths[0].len();
        } else {
            result += paths
                .iter()
                .map(|p| find_shortest_sequence(p, depth - 1, false, cursors, cache, path_cache))
                .min()
                .unwrap();
        }
        cursors[depth] = *c;
    });

    cache.insert(cache_key, result);
    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let max_depth = 2;
    let mut cache = HashMap::new();
    let mut path_cache = HashMap::new();

    let result = input.lines().fold(0, |acc, code| {
        let mut cursors = vec![b'A'; max_depth + 1];
        let len = find_shortest_sequence(
            code.as_bytes(),
            max_depth,
            true,
            &mut cursors,
            &mut cache,
            &mut path_cache,
        );

        let n = code[0..3].parse::<usize>().unwrap();
        acc + (len * n)
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let max_depth = 25;
    let mut cache = HashMap::new();
    let mut path_cache = HashMap::new();

    let result = input.lines().fold(0, |acc, code| {
        let mut cursors = vec![b'A'; max_depth + 1];
        let len = find_shortest_sequence(
            code.as_bytes(),
            max_depth,
            true,
            &mut cursors,
            &mut cache,
            &mut path_cache,
        );

        let n = code[0..3].parse::<usize>().unwrap();
        acc + (len * n)
    });

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126_384));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
