use rayon::prelude::*;
use std::{
    collections::HashMap,
    ops::{Range, RangeInclusive},
};

fn main() {
    let input = include_str!("./input.txt");

    let part_1 = process_part_1(input);
    let part_2 = process_part_2(input);

    println!("{part_1}");
    println!("{part_2}");
}
#[derive(Debug)]
struct Map {
    mappings: HashMap<RangeInclusive<i64>, i64>,
}

impl Map {
    fn map_value(&self, value: i64) -> i64 {
        for (range, shift_amount) in &self.mappings {
            if range.contains(&value) {
                return value + shift_amount;
            }
        }
        value
    }
}

fn get_seeds(input: &str) -> Vec<i64> {
    input
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_seed_ranges(input: &str) -> Vec<Range<i64>> {
    let seeds: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    seeds
        .windows(2)
        .step_by(2)
        .map(|a| a[0]..(a[0] + a[1]))
        .collect()
}
fn get_maps(input: &str) -> Vec<Map> {
    input
        .split("\n\n")
        .skip(1)
        .map(|line| {
            let (_, raw_map) = line.split_once(':').unwrap();
            let mappings: HashMap<_, _> = raw_map
                .split("\n")
                .filter(|l| !l.is_empty())
                .map(|nums| {
                    let nums: Vec<_> = nums
                        .split_whitespace()
                        .map(|num| num.parse::<i64>().unwrap())
                        .collect();
                    let range = nums[1]..=(nums[1] + nums[2] - 1);
                    let shift_amount = nums[0] - nums[1];
                    (range, shift_amount)
                })
                .collect();
            Map { mappings }
        })
        .collect()
}

fn process_part_1(input: &str) -> i64 {
    // get seed numbers from line 1
    let seeds = get_seeds(input);

    // parse remaining input into maps
    let maps = get_maps(input);
    // iterate over seeds
    let mut results = vec![];
    for seed in seeds {
        let mut result = seed;
        for map in &maps {
            result = map.map_value(result);
        }
        results.push(result);
    }
    // call min on resulting values
    *results.iter().min().unwrap()
}

fn process_part_2(input: &str) -> i64 {
    // get seed numbers from line 1
    let seed_ranges = get_seed_ranges(input);
    // parse remaining input into maps
    let maps = get_maps(input);
    // iterate over seeds
    let result = seed_ranges
        .into_par_iter()
        .map(|range| {
            range
                .into_par_iter()
                .map(|seed| {
                    let mut result = seed;
                    for map in &maps {
                        result = map.map_value(result);
                    }
                    result
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mapping() {
        let map = Map {
            mappings: [(98..=99, 50 - 98), (50..=97, 52 - 50)].into(),
        };

        let test_values = [
            (0, 0),
            (1, 1),
            (48, 48),
            (49, 49),
            (50, 52),
            (51, 53),
            (96, 98),
            (97, 99),
            (98, 50),
            (99, 51),
        ];

        for (test, expected) in test_values {
            let result = map.map_value(test);

            assert_eq!(result, expected, "{}", test);
        }
    }

    #[test]
    fn part_1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let result = process_part_1(input);

        assert_eq!(result, 35);
    }

    #[test]
    fn part_2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let result = process_part_2(input);

        assert_eq!(result, 46);
    }
}
