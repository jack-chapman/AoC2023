fn main() {
    let input = include_str!("./input.txt");

    let result_1 = solve(input, false);
    let result_2 = solve(input, true);

    println!("{result_1}");
    println!("{result_2}");
}

fn find_calibration_value(line: String) -> u32 {
    let nums: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();

    let first = nums.first().unwrap_or(&'0');
    let last = nums.last().unwrap_or(&'0');

    format!("{first}{last}").parse::<u32>().unwrap()
}

fn swap_numbers(line: &str) -> String {
    let swaps = vec![
        ("oneight", "18"),
        ("eightwo", "82"),
        ("twone", "21"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut line = line.to_string();
    for swap in &swaps {
        line = line.replace(swap.0, swap.1);
    }

    line
}

fn solve(input: &str, swap: bool) -> u32 {
    let lines = input.lines();

    let mut total = 0;

    for line in lines {
        let line = if swap {
            swap_numbers(line)
        } else {
            line.to_string()
        };
        total += find_calibration_value(line);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = solve(input, false);

        assert_eq!(result, 142);
    }

    #[test]
    fn part_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result = solve(input, true);

        println!("{result}");

        assert_eq!(result, 281);
    }
}
