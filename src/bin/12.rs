advent_of_code::solution!(12);

use std::collections::{HashSet, VecDeque};

const DIR: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

fn get_neighbors(pos: (i32, i32), map: &Vec<Vec<char>>, plant: char) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::new();

    for dir in DIR {
        let neighbor_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if (neighbor_pos.0 as usize) < map.len()
            && (neighbor_pos.1 as usize) < map[0].len()
            && map[neighbor_pos.0 as usize][neighbor_pos.1 as usize] == plant
        {
            neighbors.push(neighbor_pos);
        }
    }

    neighbors
}

fn bfs(
    pos: (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
    map: &Vec<Vec<char>>,
) -> (HashSet<(i32, i32)>, usize) {
    let mut queue = VecDeque::new();
    let plant = map[pos.0 as usize][pos.1 as usize];
    let mut region = HashSet::new();
    let mut perimeter = 0;
    visited.insert(pos);
    queue.push_back(pos);

    while let Some(current_pos) = queue.pop_front() {
        region.insert(current_pos);
        let neighbors = get_neighbors(current_pos, map, plant);
        perimeter += 4 - neighbors.len();

        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    (region, perimeter)
}

fn count_region_sides(region: &HashSet<(i32, i32)>) -> usize {
    let mut side_count = 0;

    for dir in DIR {
        let mut sides = HashSet::new();

        for pos in region {
            let neighbor_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if !region.contains(&neighbor_pos) {
                sides.insert(neighbor_pos);
            }
        }

        let mut remove = HashSet::new();
        for side in &sides {
            // Check if the side is part of a larger side by moving along orthogonal direction
            // and checking if the next position is also part of the same side
            let mut tmp = (side.0 + dir.1, side.1 + dir.0);
            while sides.contains(&tmp) {
                remove.insert(tmp);
                tmp = (tmp.0 + dir.1, tmp.1 + dir.0);
            }
        }

        side_count += sides.len() - remove.len();
    }

    side_count
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();
    let mut price = 0;

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if !visited.contains(&(x as i32, y as i32)) {
                let (region, perimeter) = bfs((x as i32, y as i32), &mut visited, &map);
                price += region.len() * perimeter;
            }
        }
    }

    Some(price)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();
    let mut price = 0;

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if !visited.contains(&(x as i32, y as i32)) {
                let (region, _) = bfs((x as i32, y as i32), &mut visited, &map);
                let sides = count_region_sides(&region);
                price += region.len() * sides;
            }
        }
    }

    Some(price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
