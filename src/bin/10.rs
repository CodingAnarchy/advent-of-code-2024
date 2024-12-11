advent_of_code::solution!(10);

use std::collections::{HashMap, HashSet};
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

// Impl add so we can use + operator on Point
impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// These are the valid direction we can move in the map.
const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
];

struct Map {
    map: HashMap<Point, isize>,
    trailheads: Vec<Point>,
}

impl Map {
    // Create the map from the given input string and return all the trail heads as well.
    fn new(input: &str) -> Map {
        let (map, heads) = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                // Convert lines and chars to points.
                line.chars()
                    .filter_map(|c| c.to_digit(10))
                    .map(|c| c as isize)
                    .enumerate()
                    .map(move |(x, c)| (Point::new(x as isize, y as isize), c))
            })
            .fold(
                // Add the points to the map and the head if they are 0.
                (HashMap::new(), Vec::new()),
                |(mut map, mut heads), (point, c)| {
                    map.insert(point, c);
                    if c == 0 {
                        heads.push(point);
                    }
                    (map, heads)
                },
            );
        Map {
            map,
            trailheads: heads,
        }
    }

    // Get the value at the given point.
    fn get(&self, point: &Point) -> Option<isize> {
        self.map.get(point).cloned()
    }

    // Get the neighbors of the given point that are in the map.
    fn neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        for d in DIRECTIONS.iter() {
            let neighbor = point + d;
            if let Some(_) = self.map.get(&neighbor) {
                neighbors.push(neighbor);
            }
        }
        neighbors
    }

    fn count_trails(&self, head: &Point) -> usize {
        // We only want to count the number of 9s we can reach from each head,
        // so we can use simple DFS and ignore nodes already visited.
        let mut nines = HashSet::new();
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        stack.push((*head, 0));
        while let Some((point, height)) = stack.pop() {
            if let Some(9) = self.get(&point) {
                nines.insert(point);
                continue;
            }

            for neighbor in self.neighbors(&point) {
                if visited.contains(&neighbor) {
                    continue;
                } else if self.get(&neighbor) != Some(height + 1) {
                    continue;
                }
                visited.insert(neighbor);
                stack.push((neighbor, height + 1));
            }
        }
        nines.len()
    }

    fn count_paths(&self, head: &Point) -> usize {
        // We want to count the unique paths to a 9 we can reach from each head,
        // so we won't ignore nodes already visited.
        let mut paths: usize = 0;
        let mut stack = Vec::new();

        stack.push((*head, 0));
        while let Some((point, height)) = stack.pop() {
            if let Some(9) = self.get(&point) {
                paths += 1;
                continue;
            }

            for neighbor in self.neighbors(&point) {
                if self.get(&neighbor) != Some(height + 1) {
                    continue;
                }
                stack.push((neighbor, height + 1));
            }
        }
        paths
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::new(input);

    Some(
        map.trailheads
            .iter()
            .map(|head| map.count_trails(head))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::new(input);

    Some(
        map.trailheads
            .iter()
            .map(|head| map.count_paths(head))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
