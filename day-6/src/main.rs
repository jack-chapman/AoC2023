fn main() {
    let input = include_str!("./input.txt");

    let part_1 = process_part_1(input);
    let part_2 = process_part_2(input);

    println!("{part_1}");
    println!("{part_2}");
}

#[derive(Debug)]
struct Race {
    distance: u64,
    time: u64,
}

fn get_numbers(line: &str) -> Vec<u64> {
    line.split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect()
}

fn get_number(line: &str) -> u64 {
    let nums: Vec<_> = line
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .collect();

    nums.join("").parse().unwrap()
}

fn get_races(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = get_numbers(lines.next().unwrap());
    let distances = get_numbers(lines.next().unwrap());

    distances
        .into_iter()
        .zip(times.into_iter())
        .map(|(distance, time)| Race { distance, time })
        .collect()
}

fn get_race(input: &str) -> Race {
    let mut lines = input.lines();
    let time = get_number(lines.next().unwrap()).to_owned();
    let distance = get_number(lines.next().unwrap()).to_owned();

    Race { time, distance }
}

fn process_part_1(input: &str) -> u64 {
    let races = get_races(input);

    races
        .iter()
        .map(|race| {
            (1..race.time).fold(0, |acc, speed| {
                let remaining_time = race.time - speed;
                let max_distance = remaining_time * speed;
                if max_distance > race.distance {
                    return acc + 1;
                }
                acc
            })
        })
        .product()
}

fn process_part_2(input: &str) -> u64 {
    let race = get_race(input);

    (1..race.time).fold(0, |acc, speed| {
        let remaining_time = race.time - speed;
        let max_distance = remaining_time * speed;
        if max_distance > race.distance {
            return acc + 1;
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let result = process_part_1(input);

        assert_eq!(result, 288);
    }

    #[test]
    fn part_2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let result = process_part_2(input);

        assert_eq!(result, 71503);
    }
}
