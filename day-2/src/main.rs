use std::str::FromStr;

fn main() {
    let input = include_str!("./input.txt");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("{part_1}");
    println!("{part_2}");
}

#[derive(Debug, PartialEq)]
enum Colour {
    Red,
    Blue,
    Green,
}

impl FromStr for Colour {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim();
        if s.ends_with(',') {
            s = &s[..s.len() - 1];
        }
        match s {
            "red" => Ok(Self::Red),
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            a => Err(format!("Cannot convert [{a}] to Colour")),
        }
    }
}

#[derive(Debug)]
struct CubeDraw {
    colour: Colour,
    count: u32,
}

impl CubeDraw {
    fn is_playable(&self) -> bool {
        match self.colour {
            Colour::Red => self.count <= 12,
            Colour::Blue => self.count <= 14,
            Colour::Green => self.count <= 13,
        }
    }
}

impl FromStr for CubeDraw {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        let count: u32 = parts[0].parse().map_err(|_| "could not parse count")?;
        let colour: Colour = parts[1].parse()?;
        Ok(Self { colour, count })
    }
}

#[derive(Debug)]
struct Turn {
    cube_draws: Vec<CubeDraw>,
}

impl Turn {
    fn is_invalid(&self) -> bool {
        let draw_validity: Vec<bool> = self
            .cube_draws
            .iter()
            .map(|draw| draw.is_playable())
            .collect();
        draw_validity.contains(&false)
    }

    fn max_of_colour(&self, colour: Colour) -> u32 {
        self.cube_draws
            .iter()
            .filter(|draw| draw.colour == colour)
            .map(|draw| draw.count)
            .max()
            .unwrap_or(0)
    }
}

impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let cube_draws: Vec<CubeDraw> = s
            .split(',')
            .filter_map(|part| part.parse::<CubeDraw>().ok())
            .collect();
        Ok(Self { cube_draws })
    }
}

#[derive(Debug)]
struct Game {
    number: u32,
    turns: Vec<Turn>,
}

impl Game {
    fn is_invalid(&self) -> bool {
        let turn_validity: Vec<bool> = self.turns.iter().map(|turn| turn.is_invalid()).collect();

        turn_validity.contains(&true)
    }

    fn power(&self) -> u32 {
        let max_red = self
            .turns
            .iter()
            .map(|turn| turn.max_of_colour(Colour::Red))
            .max()
            .unwrap();
        let max_blue = self
            .turns
            .iter()
            .map(|turn| turn.max_of_colour(Colour::Blue))
            .max()
            .unwrap();
        let max_green = self
            .turns
            .iter()
            .map(|turn| turn.max_of_colour(Colour::Green))
            .max()
            .unwrap();

        max_red * max_blue * max_green
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() == 0 {
            return Err("invalid input".to_string());
        }
        if let Some((number, turns)) = s.split_once(':') {
            let number: u32 = number
                .trim()
                .split_at(4)
                .1
                .trim()
                .parse()
                .map_err(|_| "could not parse game number")?;
            let turns: Vec<Turn> = turns
                .trim()
                .split(';')
                .filter_map(|raw_turn| match raw_turn.parse::<Turn>() {
                    Ok(turn) => Some(turn),
                    Err(e) => {
                        println!("{e}");
                        None
                    }
                })
                .collect();
            return Ok(Self { number, turns });
        }

        Err("Could not parse into Game".to_string())
    }
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| match line.parse::<Game>() {
            Ok(game) => {
                if game.is_invalid() {
                    return 0;
                }
                game.number
            }
            Err(e) => {
                eprintln!("Error parsing game: {e}");
                0
            }
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| match line.parse::<Game>() {
            Ok(game) => game.power(),
            Err(e) => {
                eprintln!("Error parsing game: {e}");
                0
            }
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = part_1(input);

        assert_eq!(result, 8);
    }

    #[test]
    fn process_part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = part_2(input);

        assert_eq!(result, 2286);
    }
}
