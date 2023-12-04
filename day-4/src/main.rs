use std::str::FromStr;

fn main() {
    let input = include_str!("./input.txt");

    let part_1 = process_part_1(input);

    println!("{part_1}");
}
#[derive(Debug)]
struct Game {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once(':').unwrap();
        let (winning, numbers) = s.split_once('|').unwrap();
        let winning_numbers = winning
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let numbers = numbers
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        Ok(Self {
            winning_numbers,
            numbers,
        })
    }
}

impl Game {
    fn score(self) -> u32 {
        let winners = self
            .numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count() as u32;

        if winners == 0 {
            return 0;
        }

        if winners == 1 {
            return 1;
        }

        2_u32.pow(winners - 1)
    }
}

fn process_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .map(|game| game.score())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = process_part_1(input);

        assert_eq!(result, 13);
    }
}
