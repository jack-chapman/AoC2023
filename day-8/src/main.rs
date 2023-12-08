use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    let part_1 = process_part_1(input);
    let part_2 = process_part_2(input);

    println!("{part_1}");
    println!("{part_2}");
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<&str, (&str, &str)>) {
    let directions: Vec<Direction> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.into())
        .collect();

    let map: HashMap<&str, (&str, &str)> = input
        .lines()
        .skip(2)
        .map(|line| {
            let (a, b) = line.split_once('=').unwrap();
            let a = a.trim();

            let (left, right) = b.trim()[1..b.len() - 2].split_once(',').unwrap();
            let right = right.trim();
            (a, (left, right))
        })
        .collect();

    (directions, map)
}

fn process_part_1(input: &str) -> u64 {
    let (directions, map) = parse_input(input);

    let mut steps = 0;
    let mut current_location = "AAA";

    for direction in directions.iter().cycle() {
        if current_location == "ZZZ" {
            break;
        }
        steps += 1;
        if let Some(location) = map.get(current_location) {
            let dir = match direction {
                Direction::Left => location.0,
                Direction::Right => location.1,
            };
            current_location = dir;
        }
    }

    steps
}

fn process_part_2(input: &str) -> u64 {
    let (directions, map) = parse_input(input);

    let start_locations: Vec<&str> = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|key| *key)
        .collect();

    start_locations
        .into_iter()
        .map(|location| {
            let mut current_location = location;
            let mut steps = 0;

            for direction in directions.iter().cycle() {
                if current_location.ends_with('Z') {
                    break;
                }
                steps += 1;
                if let Some(location) = map.get(current_location) {
                    let dir = match direction {
                        Direction::Left => location.0,
                        Direction::Right => location.1,
                    };
                    current_location = dir;
                }
            }

            steps
        })
        .fold(1, num::integer::lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_a() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let result = process_part_1(input);

        assert_eq!(result, 2);
    }

    #[test]
    fn part_1_b() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let result = process_part_1(input);

        assert_eq!(result, 6);
    }

    #[test]
    fn part_2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let result = process_part_2(input);

        assert_eq!(result, 6);
    }
}
