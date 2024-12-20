advent_of_code::solution!(20);

use std::collections::{HashMap, HashSet};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
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

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, other: i32) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 1, y: 0 },
];
const DEPRECATED_CHEAT_DURATION: i32 = 2;
const CHEAT_DURATION: i32 = 20;

struct RaceSetup {
    obstacles: HashSet<Point>,
    start: Point,
    end: Point,
}

impl RaceSetup {
    fn new(input: &str) -> Self {
        let mut obstacles = HashSet::new();
        let mut start = Point { x: 0, y: 0 };
        let mut end = Point { x: 0, y: 0 };

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let position = Point {
                    x: x as i32,
                    y: y as i32,
                };
                match c {
                    '#' => {
                        obstacles.insert(position);
                    }
                    'S' => {
                        start = position;
                    }
                    'E' => {
                        end = position;
                    }
                    _ => (),
                };
            });
        });

        Self {
            obstacles,
            start,
            end,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let race = RaceSetup::new(input);

    let mut path = HashMap::from([(race.start, 0)]);
    let mut pos = race.start;

    while pos != race.end {
        for dir in DIRECTIONS {
            let next_position = pos + &dir;
            if !path.contains_key(&next_position) && !race.obstacles.contains(&next_position) {
                path.insert(next_position, *path.get(&pos).unwrap() + 1);
                pos = next_position;
            }
        }
    }

    let mut count = 0;
    #[cfg(test)]
    let saved = 20;
    #[cfg(not(test))]
    let saved = 100;

    for (position, cost) in path.iter() {
        for dir in DIRECTIONS {
            let next_position = *position + &(dir * 2);
            if let Some(next_cost) = path.get(&next_position) {
                if next_cost - cost >= saved + DEPRECATED_CHEAT_DURATION {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let race = RaceSetup::new(input);

    let mut path = vec![race.start];
    let mut visited = HashSet::from([race.start]);
    let mut position = race.start;

    while position != race.end {
        for dir in DIRECTIONS {
            let next_position = position + &dir;
            if !visited.contains(&next_position) && !race.obstacles.contains(&next_position) {
                visited.insert(next_position);
                path.push(next_position);
                position = next_position;
            }
        }
    }

    let mut count = 0;
    #[cfg(test)]
    let saved = 50;
    #[cfg(not(test))]
    let saved = 100;

    for cheat_start_index in 0..path.len() {
        for cheat_end_index in cheat_start_index + 1..path.len() {
            let distance = path[cheat_start_index].distance(&path[cheat_end_index]);

            if distance <= CHEAT_DURATION
                && (cheat_end_index - cheat_start_index) as i32 >= saved + distance
            {
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            result,
            Some(32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3)
        );
    }
}
