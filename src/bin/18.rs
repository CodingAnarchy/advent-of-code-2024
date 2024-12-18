advent_of_code::solution!(18);

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
    fn new(grid_size: usize) -> Self {
        let vec = vec!['.'; grid_size * grid_size];
        Self {
            vec,
            width: grid_size,
            height: grid_size,
        }
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

pub fn part_one(input: &str) -> Option<usize> {
    #[cfg(test)]
    let grid_size = 7;
    #[cfg(test)]
    let steps = 12;

    #[cfg(not(test))]
    let grid_size = 71;
    #[cfg(not(test))]
    let steps = 1024;

    let coords = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect::<Vec<Point>>();

    let mut grid = Grid::new(grid_size);
    for i in 0..steps {
        grid[coords[i]] = '#';
    }

    // Dijkstra's algorithm
    let mut buckets = vec![Vec::new(); 101];
    let mut seen = Grid {
        vec: vec![[u32::MAX; 4]; grid.vec.len()],
        width: grid.width,
        height: grid.height,
    };
    let mut cost = 0;
    let mut lowest = u32::MAX;
    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: grid_size as isize - 1,
        y: grid_size as isize - 1,
    };

    buckets[0].push((start, 0));
    seen[start][0] = 0;

    while lowest == u32::MAX {
        let index = (cost % 101) as usize;

        while let Some((point, direction)) = buckets[index].pop() {
            if point == end {
                lowest = cost;
                break;
            }

            let left = (direction + 3) % 4;
            let right = (direction + 1) % 4;
            let next = [
                (point + &DIRECTIONS[direction], direction, cost + 1),
                (point, left, cost),
                (point, right, cost),
            ];

            for (next_pos, next_dir, next_cost) in next {
                if next_pos.x < 0
                    || next_pos.y < 0
                    || next_pos.x >= grid_size as isize
                    || next_pos.y >= grid_size as isize
                {
                    continue;
                }

                if grid[next_pos] != '#' && next_cost < seen[next_pos][next_dir] {
                    let index = (next_cost % 101) as usize;
                    buckets[index].push((next_pos, next_dir));
                    seen[next_pos][next_dir] = next_cost;
                }
            }
        }

        cost += 1;
    }

    Some(lowest as usize)
}

fn find(grid: &mut Grid<Point>, point: Point) -> Point {
    if point != grid[point] {
        grid[point] = find(grid, grid[point]);
    }
    grid[point]
}

fn union(grid: &Grid<char>, parent: &mut Grid<Point>, p: Point, dir: &Point) {
    let mov = p + dir;
    if mov.x >= 0
        && mov.y >= 0
        && mov.x < grid.width as isize
        && mov.y < grid.height as isize
        && grid[mov] == '.'
    {
        let idx = find(parent, p);
        parent[idx] = find(parent, mov);
    }
}

fn has_valid_path(grid: &Grid<char>) -> bool {
    let parent_vec = (0..grid.vec.len())
        .map(|i| Point {
            x: i as isize % grid.width as isize,
            y: i as isize / grid.width as isize,
        })
        .collect::<Vec<Point>>();

    let mut parent = Grid {
        vec: parent_vec,
        width: grid.width,
        height: grid.height,
    };

    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point {
                x: x as isize,
                y: y as isize,
            };
            if grid[p] == '.' {
                for dir in DIRECTIONS {
                    union(&grid, &mut parent, p, &dir);
                }
            }
        }
    }

    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: grid.width as isize - 1,
        y: grid.height as isize - 1,
    };

    find(&mut parent, start) == find(&mut parent, end)
}

pub fn part_two(input: &str) -> Option<String> {
    #[cfg(test)]
    let grid_size = 7;

    #[cfg(not(test))]
    let grid_size = 71;

    let coords = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect::<Vec<Point>>();

    let mut grid = Grid::new(grid_size);
    for i in 0..coords.len() {
        grid[coords[i]] = '#';
        if !has_valid_path(&grid) {
            return Some(format!("{},{}", coords[i].x, coords[i].y));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
