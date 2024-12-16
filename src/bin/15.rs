advent_of_code::solution!(15);

use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
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
    Point { x: 0, y: -1 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 1, y: 0 },
];

struct Warehouse {
    grid: Vec<Vec<char>>,
    robot: Point,
    moves: Vec<Point>,
}

impl Warehouse {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        let moves = input
            .lines()
            .skip_while(|line| line.starts_with("#") || line.is_empty())
            .flat_map(|line| {
                line.chars()
                    .map(|c| match c {
                        '^' => DIRECTIONS[0],
                        'v' => DIRECTIONS[1],
                        '<' => DIRECTIONS[2],
                        '>' => DIRECTIONS[3],
                        _ => unreachable!("Invalid move: {}", c),
                    })
                    .collect::<Vec<Point>>()
            })
            .collect::<Vec<Point>>();

        let robot = grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(
                    |(x, &cell)| {
                        if cell == '@' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap();

        Self {
            grid,
            robot: Point {
                x: robot.0 as isize,
                y: robot.1 as isize,
            },
            moves,
        }
    }

    fn solve(&mut self) -> usize {
        'outer: for m in self.moves.iter() {
            let mut queue = VecDeque::from([self.robot]);
            let mut seen = HashSet::new();
            while let Some(robot) = queue.pop_front() {
                if !seen.insert(robot) {
                    continue;
                }

                let robot_2 = &robot + m;

                match self.grid[robot_2.y as usize][robot_2.x as usize] {
                    '#' => continue 'outer,
                    'O' => queue.push_back(robot_2),
                    '[' => {
                        let box_extension = Point {
                            x: robot_2.x + 1,
                            y: robot_2.y,
                        };
                        queue.extend([robot_2, box_extension]);
                    }
                    ']' => {
                        let box_extension = Point {
                            x: robot_2.x - 1,
                            y: robot_2.y,
                        };
                        queue.extend([robot_2, box_extension]);
                    }
                    _ => continue,
                }
            }

            let boxes = seen
                .iter()
                .sorted_by_key(|p| (self.robot.x.abs_diff(p.x), self.robot.y.abs_diff(p.y)))
                .rev()
                .collect::<Vec<&Point>>();

            for b in boxes.iter() {
                let p = *b + m;
                self.grid[p.y as usize][p.x as usize] = self.grid[b.y as usize][b.x as usize];
                self.grid[b.y as usize][b.x as usize] = '.';
            }
            self.robot = &self.robot + m;
        }

        (0..self.grid.len())
            .cartesian_product(0..self.grid[0].len())
            .filter(|p| matches!(self.grid[p.0][p.1], 'O' | '['))
            .map(|p| p.0 * 100 + p.1)
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut warehouse = Warehouse::new(input);
    Some(warehouse.solve())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut wide_input = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.bytes()
                .flat_map(|c| match c {
                    b'@' => b"@.",
                    b'O' => b"[]",
                    b'#' => b"##",
                    b'.' => b"..",
                    _ => unreachable!("Invalid character: {}", c as char),
                })
                .copied()
                .collect::<Vec<u8>>()
        })
        .map(|line| String::from_utf8(line).unwrap())
        .join("\n");

    let moves = input
        .lines()
        .skip_while(|line| line.starts_with("#") || line.is_empty())
        .collect::<String>();

    wide_input.push_str("\n\n");
    wide_input.push_str(&moves);

    let mut warehouse = Warehouse::new(&wide_input);
    Some(warehouse.solve())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_small() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10_092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
