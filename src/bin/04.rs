advent_of_code::solution!(4);

use itertools::Itertools;

struct Matrix {
    pub data: Vec<Vec<char>>,
}

impl Matrix {
    fn new(input: &str) -> Self {
        let data = input.lines().map(|line| line.chars().collect()).collect();

        Self { data }
    }

    fn width(&self) -> usize {
        self.data.get(0).map_or(0, |row| row.len())
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        self.data.get(x)?.get(y).copied()
    }
}

fn is_forward_xmas(matrix: &Matrix, x: usize, y: usize) -> bool {
    if y + 3 >= matrix.width() {
        return false;
    }

    let horizontal: [char; 4] = matrix.data[x][y..=y + 3].try_into().unwrap();

    horizontal == ['X', 'M', 'A', 'S'] || horizontal == ['S', 'A', 'M', 'X']
}

fn is_vertical_forward_xmas(matrix: &Matrix, x: usize, y: usize) -> bool {
    if x + 3 >= matrix.height() {
        return false;
    }

    let vertical = matrix.data[x..=x + 3]
        .iter()
        .map(|r| r.get(y).unwrap().clone())
        .collect::<Vec<char>>();

    vertical == vec!['X', 'M', 'A', 'S'] || vertical == vec!['S', 'A', 'M', 'X']
}

fn is_se_diagonal_xmas(matrix: &Matrix, x: usize, y: usize) -> bool {
    if x + 3 >= matrix.height() || y + 3 >= matrix.width() {
        return false;
    }

    let diagonal = (0..=3)
        .map(|i| matrix.data[x + i][y + i])
        .collect::<Vec<char>>();

    diagonal == vec!['X', 'M', 'A', 'S'] || diagonal == vec!['S', 'A', 'M', 'X']
}

fn is_sw_diagonal_xmas(matrix: &Matrix, x: usize, y: usize) -> bool {
    if x + 3 >= matrix.height() || y < 3 {
        return false;
    }

    let diagonal = (0..=3)
        .map(|i| matrix.data[x + i][y - i])
        .collect::<Vec<char>>();

    diagonal == vec!['X', 'M', 'A', 'S'] || diagonal == vec!['S', 'A', 'M', 'X']
}

fn is_x_mas(matrix: &Matrix, x: usize, y: usize) -> bool {
    let sw_diagonal = (0..=2)
        .map(|i| matrix.data[x - 1 + i][y - 1 + i])
        .sorted()
        .collect::<Vec<char>>();

    let se_diagonal = (0..=2)
        .map(|i| matrix.data[x - 1 + i][y + 1 - i])
        .sorted()
        .collect::<Vec<char>>();

    (sw_diagonal == vec!['A', 'M', 'S']) && (se_diagonal == vec!['A', 'M', 'S'])
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = Matrix::new(input);
    let mut total_xmas = 0;

    for x in 0..matrix.height() {
        for y in 0..matrix.width() {
            match matrix.get(x, y) {
                Some('X') | Some('S') => {
                    if is_forward_xmas(&matrix, x, y) {
                        total_xmas += 1;
                    }

                    if is_vertical_forward_xmas(&matrix, x, y) {
                        total_xmas += 1;
                    }

                    if is_se_diagonal_xmas(&matrix, x, y) {
                        total_xmas += 1;
                    }

                    if is_sw_diagonal_xmas(&matrix, x, y) {
                        total_xmas += 1;
                    }
                }
                _ => {}
            }
        }
    }

    Some(total_xmas)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = Matrix::new(input);
    let mut total_xmas = 0;

    for x in 1..matrix.height() - 1 {
        for y in 1..matrix.width() - 1 {
            match matrix.get(x, y) {
                Some('A') => {
                    if is_x_mas(&matrix, x, y) {
                        total_xmas += 1;
                    }
                }
                _ => {}
            }
        }
    }

    Some(total_xmas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
