advent_of_code::solution!(16);

use std::collections::VecDeque;
use std::ops::{Add, Index, IndexMut, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

// Impl add so we can use + operator on Point
impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<&Point> for Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

const DIRECTIONS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 },
];

struct Grid<T> {
    vec: Vec<T>,
    width: usize,
    height: usize,
}

impl Grid<char> {
    fn new(input: &str) -> Self {
        let vec = input.chars().collect();
        // +1 to account for newline in the raw bytes
        let width = input.lines().next().unwrap().len() + 1;
        let height = input.lines().count();

        Self { vec, width, height }
    }
}

impl<T: Copy + PartialEq + std::fmt::Debug> Grid<T> {
    fn find(&self, needle: T) -> Option<Point> {
        self.vec.iter().position(|&x| x == needle).map(|i| Point {
            x: (i % self.width) as isize,
            y: (i / self.width) as isize,
        })
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        let idx = point.x + point.y * self.width as isize;
        &self.vec[idx as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        let idx = point.x + point.y * self.width as isize;
        &mut self.vec[idx as usize]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();

    // Dijkstra's algorithm
    let mut buckets = vec![Vec::new(); 1001];
    let mut seen = Grid {
        vec: vec![[u32::MAX; 4]; grid.vec.len()],
        width: grid.width,
        height: grid.height,
    };
    let mut cost = 0;
    let mut lowest = u32::MAX;

    buckets[0].push((start, 0));
    seen[start][0] = 0;

    while lowest == u32::MAX {
        let index = (cost % 1001) as usize;

        while let Some((point, direction)) = buckets[index].pop() {
            if point == end {
                lowest = cost;
                break;
            }

            let left = (direction + 3) % 4;
            let right = (direction + 1) % 4;
            let next = [
                (point + &DIRECTIONS[direction], direction, cost + 1),
                (point, left, cost + 1000),
                (point, right, cost + 1000),
            ];

            for (next_pos, next_dir, next_cost) in next {
                if grid[next_pos] != '#' && next_cost < seen[next_pos][next_dir] {
                    let index = (next_cost % 1001) as usize;
                    buckets[index].push((next_pos, next_dir));
                    seen[next_pos][next_dir] = next_cost;
                }
            }
        }

        cost += 1;
    }

    Some(lowest)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::new(input);
    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();

    // Dijkstra's algorithm
    let mut buckets = vec![Vec::new(); 1001];
    let mut seen = Grid {
        vec: vec![[u32::MAX; 4]; grid.vec.len()],
        width: grid.width,
        height: grid.height,
    };
    let mut cost = 0;
    let mut lowest = u32::MAX;

    buckets[0].push((start, 0));
    seen[start][0] = 0;

    while lowest == u32::MAX {
        let index = (cost % 1001) as usize;

        while let Some((point, direction)) = buckets[index].pop() {
            if point == end {
                lowest = cost;
                break;
            }

            let left = (direction + 3) % 4;
            let right = (direction + 1) % 4;
            let next = [
                (point + &DIRECTIONS[direction], direction, cost + 1),
                (point, left, cost + 1000),
                (point, right, cost + 1000),
            ];

            for (next_pos, next_dir, next_cost) in next {
                if grid[next_pos] != '#' && next_cost < seen[next_pos][next_dir] {
                    let index = (next_cost % 1001) as usize;
                    buckets[index].push((next_pos, next_dir));
                    seen[next_pos][next_dir] = next_cost;
                }
            }
        }

        cost += 1;
    }

    // Backwards BFS to find a good seat
    let mut todo = VecDeque::new();
    let mut path = Grid {
        vec: vec![false; grid.vec.len()],
        width: grid.width,
        height: grid.height,
    };

    for direction in 0..4 {
        if seen[end][direction] == lowest {
            todo.push_back((end, direction, lowest));
        }
    }

    while let Some((pos, dir, cost)) = todo.pop_front() {
        path[pos] = true;

        if pos == start {
            continue;
        }

        // Reverse direction and subtract cost
        let left = (dir + 3) % 4;
        let right = (dir + 1) % 4;
        let next = if cost > 1000 {
            vec![
                (pos - &DIRECTIONS[dir], dir, cost - 1),
                (pos, left, cost - 1000),
                (pos, right, cost - 1000),
            ]
        } else {
            vec![(pos - &DIRECTIONS[dir], dir, cost - 1)]
        };

        for (next_pos, next_dir, next_cost) in next {
            if next_cost == seen[next_pos][next_dir] {
                todo.push_back((next_pos, next_dir, next_cost));
                seen[next_pos][next_dir] = u32::MAX;
            }
        }
    }

    Some(path.vec.iter().filter(|&&x| x).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
