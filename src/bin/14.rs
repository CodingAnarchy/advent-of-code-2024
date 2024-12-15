advent_of_code::solution!(14);

use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn new(input: &str) -> Self {
        let (pos, vel) = input.split_once(" ").unwrap();
        let (pos_x, pos_y) = pos.strip_prefix("p=").unwrap().split_once(",").unwrap();
        let (vel_x, vel_y) = vel.strip_prefix("v=").unwrap().split_once(",").unwrap();

        Robot {
            position: Point {
                x: pos_x.parse().unwrap(),
                y: pos_y.parse().unwrap(),
            },
            velocity: Point {
                x: vel_x.parse().unwrap(),
                y: vel_y.parse().unwrap(),
            },
        }
    }

    fn move_100_seconds(&mut self, max_x: i32, max_y: i32) {
        let new_x = self.position.x + (100 * self.velocity.x);
        let new_y = self.position.y + (100 * self.velocity.y);

        let mod_x = new_x % max_x;
        if mod_x < 0 {
            self.position.x = max_x + mod_x;
        } else {
            self.position.x = mod_x;
        }

        let mod_y = new_y % max_y;
        if mod_y < 0 {
            self.position.y = max_y + mod_y;
        } else {
            self.position.y = mod_y;
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut robots = input.lines().map(Robot::new).collect::<Vec<_>>();
    #[cfg(test)]
    let (max_x, max_y) = (11, 7);
    #[cfg(not(test))]
    let (max_x, max_y) = (101, 103);

    robots
        .iter_mut()
        .for_each(|robot| robot.move_100_seconds(max_x, max_y));

    let mid_x = max_x / 2;
    let mid_y = max_y / 2;

    let mut quadrants = HashMap::new();
    robots
        .iter()
        .chunk_by(|robot| {
            let x = robot.position.x;
            let y = robot.position.y;

            if x < mid_x && y < mid_y {
                1
            } else if x > mid_x && y < mid_y {
                2
            } else if x < mid_x && y > mid_y {
                3
            } else if x > mid_x && y > mid_y {
                4
            } else {
                0
            }
        })
        .into_iter()
        .for_each(|(key, group)| {
            if key == 0 {
                // filter out the robots in the quadrant division lines
                return;
            }

            let robots = group.count();
            match quadrants.entry(key) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += robots;
                }
                Entry::Vacant(entry) => {
                    entry.insert(robots);
                }
            }
        });

    let safety_factor = quadrants.values().product::<usize>();

    Some(safety_factor)
}

pub fn part_two(input: &str) -> Option<usize> {
    let robots = input.lines().map(Robot::new).collect::<Vec<_>>();
    #[cfg(test)]
    return None; // No example for part two

    const MAX_X: i32 = 101;
    const MAX_Y: i32 = 103;

    // Search for times mod 101 when the tree could possibly exist using x coordinates only.
    // and times mod 103 when the tree could possibly exist using y coordinates only.
    let mut rows = Vec::new();
    let mut cols = Vec::new();

    for time in 0..MAX_Y {
        let mut xs = [0; MAX_X as usize];
        let mut ys = [0; MAX_Y as usize];

        for robot in robots.iter() {
            let x = (robot.position.x + time * robot.velocity.x.rem_euclid(MAX_X)) % MAX_X;
            xs[x as usize] += 1;
            let y = (robot.position.y + time * robot.velocity.y.rem_euclid(MAX_Y)) % MAX_Y;
            ys[y as usize] += 1;
        }

        // Tree bounding box is 31x33.
        if time < MAX_X && xs.iter().filter(|&&c| c >= 33).count() >= 2 {
            cols.push(time);
        }
        if ys.iter().filter(|&&c| c >= 31).count() >= 2 {
            rows.push(time);
        }
    }

    // If there's only one combination then return answer.
    // if rows.len() == 1 && columns.len() == 1 {
    let t = cols[0];
    let u = rows[0];
    // Combine indices using the Chinese Remainder Theorem to get index mod 10403.
    let res = (5253 * t + 5151 * u) % 10403;
    return Some(res as usize);
    //}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
