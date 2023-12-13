use itertools::{repeat_n, Itertools};
use rayon::prelude::*;

fn main() {
    let input = include_str!("./input.txt");

    let part_1 = process_part_1(input);

    println!("{part_1}");
}

fn process_part_1(input: &str) -> u32 {
    let lines: Vec<Line> = input.lines().map(|line| line.into()).collect();

    lines
        .par_iter()
        .fold(|| 0, |acc, line| acc + line.solve())
        .sum()
}

struct Line {
    unknown_count: u32,
    chars: Vec<char>,
    pattern: Vec<u32>,
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let (chars, pattern) = value.split_once(' ').expect("Could not split line");

        let chars: Vec<_> = chars.trim().chars().collect();
        let pattern = pattern
            .trim()
            .split(',')
            .map(|s| s.parse().expect("Could not parse pattern"))
            .collect();

        let unknown_count = chars.iter().copied().filter(|c| *c == '?').count() as u32;

        Self {
            chars,
            pattern,
            unknown_count,
        }
    }
}

impl Line {
    fn solve(&self) -> u32 {
        let combos = self.get_combos();
        combos
            .par_iter()
            .filter(|combo| self.check_combo(combo))
            .count() as u32
    }
    fn get_combos(&self) -> Vec<String> {
        repeat_n([".", "#"].into_iter(), self.unknown_count as usize)
            .multi_cartesian_product()
            .map(|s| s.join(""))
            .collect()
    }
    fn check_combo(&self, combo: &str) -> bool {
        let mut combo_iter = combo.chars();
        let filled_combo: String = self
            .chars
            .iter()
            .copied()
            .map(|c| match c {
                '?' => combo_iter.next().expect("should always have space"),
                value => value,
            })
            .collect();

        let counts: Vec<u32> = filled_combo
            .chars()
            .group_by(|c| *c == '#')
            .into_iter()
            .filter_map(|(is_unknown, combo)| {
                is_unknown.then_some(combo.into_iter().count() as u32)
            })
            .collect();

        self.pattern[..] == counts[..]
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    #[test]
    fn part_1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let result = process_part_1(input);

        assert_eq!(result, 21);
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn part_1_cases(#[case] input: &str, #[case] expected: u32) {
        let line: Line = input.into();
        let result = line.solve();

        assert_eq!(result, expected);
    }
}
