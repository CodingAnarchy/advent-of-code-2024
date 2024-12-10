advent_of_code::solution!(6);

use std::collections::{HashMap, HashSet};
use std::iter::{self, Iterator};
use std::ops::Range;

use rayon::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn moved(&self, dir: Direction) -> Option<Position> {
        Some(match dir {
            Direction::Up => Position {
                x: self.x.checked_sub(1)?,
                y: self.y,
            },
            Direction::Down => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Position {
                x: self.x,
                y: self.y.checked_sub(1)?,
            },
            Direction::Right => Position {
                x: self.x,
                y: self.y + 1,
            },
        })
    }

    pub fn distance(&self, other: &Position) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Guard {
    pos: Position,
    dir: Direction,
}

#[derive(Clone, Debug)]
pub struct Grid {
    guard: Guard,
    obstacles: HashSet<Position>,
    collisions: HashSet<(Position, Direction)>,
    x_max: Range<usize>,
    y_max: Range<usize>,
}

impl Grid {
    pub fn from_string(raw: &str) -> Result<Grid, String> {
        let mut map: HashMap<char, HashSet<Position>> = HashMap::new();
        for (x, line) in raw.lines().enumerate() {
            for (y, c) in line.chars().enumerate() {
                map.entry(c).or_default().insert(Position { x, y });
            }
        }

        match map.get(&'^') {
            None => Err("Error: No guard!".to_owned()),
            Some(g) if g.len() > 1 => Err("Error: More than one guard, You're cooked!".to_owned()),
            Some(g) => Ok(Grid {
                guard: Guard {
                    pos: *g.iter().next().unwrap(),
                    dir: Direction::Up,
                },
                obstacles: map[&'#'].clone(),
                collisions: HashSet::new(),
                x_max: 0..raw.lines().count(),
                y_max: 0..raw.lines().last().unwrap().chars().count(),
            }),
        }
    }

    fn with_new_obstacle(&self, new_obj: Position) -> Self {
        let mut new = self.clone();
        new.obstacles.insert(new_obj);
        new
    }

    fn next_index(&self) -> Option<Position> {
        let i = self.guard.pos.moved(self.guard.dir)?;
        (self.x_max.contains(&i.x) && self.y_max.contains(&i.y)).then_some(i)
    }

    pub fn would_loop(&mut self) -> bool {
        if let Some(final_state) = self.last() {
            if let Some(next) = final_state.pos.moved(final_state.dir) {
                return self.obstacles.contains(&next);
            }
        }
        false
    }
}

impl Iterator for Grid {
    type Item = Guard;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.next_index()?;
        if self.obstacles.contains(&i) {
            if !self.collisions.insert((i, self.guard.dir)) {
                // Loop detected!
                return None;
            };
            self.guard.dir = self.guard.dir.turn();
        } else {
            self.guard.pos = i;
        };
        Some(self.guard)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_string(input).unwrap();

    Some(
        HashSet::<Position>::from_iter(iter::once(grid.guard.pos).chain(grid.map(|g| g.pos))).len()
            as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_string(input).unwrap();

    Some(
        HashSet::<Position>::from_iter(grid.clone().map(|step| step.pos))
            .par_iter() // for each position the guard was in
            .filter(|new_obs| grid.with_new_obstacle(**new_obs).would_loop()) // try adding a new obstacle and see if it causes a loop
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
