use std::{
    iter::from_fn,
    ops::{Add, Sub},
};

use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");

    let part_1 = process_part_1(input);
    let part_2 = process_part_2(input);

    println!("{part_1}");
    println!("{part_2}");
}

fn process_part_1(input: &str) -> i64 {
    process(input, 2)
}

fn process_part_2(input: &str) -> i64 {
    process(input, 1000000)
}

fn process(input: &str, multiplier: i64) -> i64 {
    let (empty_rows, empty_columns) = find_empty_rows_and_columns(input);

    let galaxies = find_galaxies(input);

    let count: i64 = galaxies
        .iter()
        .combinations(2)
        .map(|s| {
            let g_a_expanded = expand_point(s[0], &empty_rows, &empty_columns, multiplier);
            let g_b_expanded = expand_point(s[1], &empty_rows, &empty_columns, multiplier);
            let vector = (g_b_expanded - g_a_expanded).abs();
            let distance = vector.x + vector.y;
            distance
        })
        .sum();

    count
}

fn expand_point(
    point: &Point,
    empty_rows: &Vec<usize>,
    empty_columns: &Vec<usize>,
    multiplier: i64,
) -> Point {
    let expand_steps_row = empty_rows
        .iter()
        .position(|row| row > &(point.y as usize))
        .unwrap_or(empty_rows.len());

    let expand_steps_column = empty_columns
        .iter()
        .position(|column| column > &(point.x as usize))
        .unwrap_or(empty_columns.len());

    *point
        + Point::new(
            expand_steps_column as i64 * (multiplier - 1),
            expand_steps_row as i64 * (multiplier - 1),
        )
}

fn find_empty_rows_and_columns(input: &str) -> (Vec<usize>, Vec<usize>) {
    let empty_rows: Vec<usize> = input
        .lines()
        .enumerate()
        .filter_map(|(index, line)| line.chars().all(|c| c == '.').then_some(index))
        .collect();

    let mut columns: Vec<_> = input.lines().map(|line| line.chars()).collect();

    let empty_columns: Vec<usize> = from_fn(move || {
        let mut items = vec![];
        for iter in &mut columns {
            match iter.next() {
                Some(item) => {
                    items.push(item);
                }
                None => return None,
            }
        }
        Some(items)
    })
    .enumerate()
    .filter_map(|(index, column)| column.iter().all(|c| *c == '.').then_some(index))
    .collect();

    (empty_rows, empty_columns)
}

fn find_galaxies(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Point::new(x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let result = process_part_1(input);

        assert_eq!(result, 374);
    }
}
