use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");

    let part_1 = process_part_1(input);
    let part_2 = process_part_2(input);

    println!("{part_1}");
    println!("{part_2}");
}

#[derive(Debug, PartialEq)]
enum SchemaItem {
    Blank,
    Symbol,
    Gear,
    Number(u32),
}

impl From<char> for SchemaItem {
    fn from(value: char) -> Self {
        if value.is_numeric() {
            return Self::Number(value.to_digit(10).unwrap());
        }
        if value == '.' {
            return Self::Blank;
        }
        if value == '*' {
            return Self::Gear;
        }
        Self::Symbol
    }
}

fn parse_into_grid(input: &str) -> Vec<Vec<SchemaItem>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect()
}

fn collect_numbers(grid: &Vec<Vec<SchemaItem>>) -> HashMap<((usize, usize), (usize, usize)), u32> {
    let mut map = HashMap::new();
    for (x, row) in grid.iter().enumerate() {
        let groups = row.iter().enumerate().group_by(|(_, item)| match item {
            SchemaItem::Number(_) => true,
            _ => false,
        });
        for group in &groups {
            let group: Vec<_> = group
                .1
                .filter_map(|item| match item.1 {
                    SchemaItem::Number(x) => Some((item.0, x)),
                    _ => None,
                })
                .collect();
            if group.len() > 0 {
                let num = group.iter().map(|g| g.1).join("");
                let num = num.parse::<u32>().unwrap();
                let start = (x, group[0].0);
                let end = (x, group[group.len() - 1].0);
                map.insert((start, end), num);
            }
        }
    }
    map.clone()
}

fn collect_gears(grid: &Vec<Vec<SchemaItem>>) -> Vec<(usize, usize)> {
    let mut gears = vec![];
    for (x, row) in grid.iter().enumerate() {
        for (y, item) in row.iter().enumerate() {
            if *item == SchemaItem::Gear {
                gears.push((x, y));
            }
        }
    }
    gears.clone()
}

fn find_overlaps(
    numbers: HashMap<((usize, usize), (usize, usize)), u32>,
    gears: Vec<(usize, usize)>,
    grid: Vec<Vec<SchemaItem>>,
) -> Vec<(u32, u32)> {
    let mut result = vec![];
    for gear in gears {
        let search_positions = [
            (gear.0.saturating_sub(1), gear.1.saturating_sub(1)),
            (gear.0.saturating_sub(1), gear.1),
            (gear.0.saturating_sub(1), gear.1 + 1),
            (gear.0, gear.1.saturating_sub(1)),
            (gear.0, gear.1 + 1),
            (gear.0 + 1, gear.1.saturating_sub(1)),
            (gear.0 + 1, gear.1),
            (gear.0 + 1, gear.1 + 1),
        ];

        let mut number_positions = vec![];
        for search_position in search_positions {
            if search_position.0 > grid.len() || search_position.1 > grid[0].len() {
                continue;
            }
            let test = &grid[search_position.0][search_position.1];
            match test {
                SchemaItem::Number(_) => number_positions.push(search_position),
                _ => (),
            }
        }
        let mut number_matches = vec![];
        for ((start, end), num) in numbers.iter() {
            if number_positions.contains(&start) || number_positions.contains(&end) {
                number_matches.push(*num);
            }
        }

        if number_matches.len() == 2 {
            result.push((number_matches[0], number_matches[1]));
        }
    }
    result
}

fn process_part_1(input: &str) -> u32 {
    let grid = parse_into_grid(input);
    let numbers = collect_numbers(&grid);
    // for each number in the list above, check to see if they're adjacent to symbols in grid
    let result: u32 = numbers
        .iter()
        .map(|((start, end), number)| {
            // start.0 - 1 to end.0 + 1
            //      check [x][start.1] and [x][end.1]
            //      start.1 - 1 to end.1 - 1
            //      start.1 + 1 to end.1 + 1
            //      if grid[x][y] === symbol
            //          inc counter by number
            for x in start.0.saturating_sub(1)..=(end.0 + 1) {
                if x > grid.len() - 1 {
                    continue;
                }

                let item = &grid[x][start.1];
                if *item == SchemaItem::Symbol || *item == SchemaItem::Gear {
                    return *number;
                }
                let item = &grid[x][end.1];
                if *item == SchemaItem::Symbol || *item == SchemaItem::Gear {
                    return *number;
                }

                for y in start.1.saturating_sub(1)..=end.1.saturating_sub(1) {
                    if y > grid[x].len() {
                        continue;
                    }
                    let item = &grid[x][y];
                    if *item == SchemaItem::Symbol || *item == SchemaItem::Gear {
                        return *number;
                    }
                }

                for y in (start.1 + 1)..=(end.1 + 1) {
                    if y > grid[x].len() - 1 {
                        continue;
                    }
                    let item = &grid[x][y];
                    if *item == SchemaItem::Symbol || *item == SchemaItem::Gear {
                        return *number;
                    }
                }
            }
            0
        })
        .sum();
    result
}

fn process_part_2(input: &str) -> u32 {
    let grid = parse_into_grid(input);
    let numbers = collect_numbers(&grid);
    let gears = collect_gears(&grid);
    let overlaps = find_overlaps(numbers, gears, grid);
    overlaps.iter().map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = process_part_1(input);

        assert_eq!(result, 4361);
    }

    #[test]
    fn part_2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = process_part_2(input);

        assert_eq!(result, 467835);
    }
}
