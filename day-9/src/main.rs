use itertools::{Itertools, Position};
use rayon::prelude::*;

fn main() {
    let input = include_str!("./input.txt");

    let part_1 = process_part_1(input);
    let part_2 = process_part_2(input);

    println!("{part_1}");
    println!("{part_2}");
}

fn process_part_1(input: &str) -> i64 {
    let result = input
        .par_lines()
        .map(|line| {
            let mut nums = collect_initial_nums(line);

            let mut end_nums: Vec<i64> = vec![];

            loop {
                if nums.iter().all(|num| *num == 0) {
                    break;
                }
                nums = find_new_nums(nums, &mut end_nums, Position::Last);
            }

            end_nums.iter().sum::<i64>()
        })
        .sum::<i64>();

    result
}

fn process_part_2(input: &str) -> i64 {
    let result = input
        .par_lines()
        .map(|line| {
            let mut nums = collect_initial_nums(line);

            let mut start_nums: Vec<i64> = vec![];

            loop {
                if nums.iter().all(|num| *num == 0) {
                    break;
                }
                nums = find_new_nums(nums, &mut start_nums, Position::First);
            }

            start_nums.iter().sum::<i64>()
        })
        .sum::<i64>();

    result
}

fn collect_initial_nums(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn find_new_nums(nums: Vec<i64>, focus_nums: &mut Vec<i64>, focus_position: Position) -> Vec<i64> {
    nums.iter()
        .tuple_windows()
        .with_position()
        .map(|(position, (left, right))| match focus_position {
            Position::First => {
                if position == focus_position || position == Position::Only {
                    focus_nums.push(*left);
                }
                left - right
            }
            Position::Last => {
                if position == focus_position || position == Position::Only {
                    focus_nums.push(*right);
                }
                right - left
            }
            _ => unreachable!(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = process_part_1(input);

        assert_eq!(result, 114);
    }

    #[test]
    fn part_2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = process_part_2(input);

        assert_eq!(result, 2);
    }
}
