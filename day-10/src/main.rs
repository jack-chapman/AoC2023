fn main() {
    let input = include_str!("./input.txt");

    let part_1 = process_part_1(input);
    let part_2 = process_part_2(input);

    println!("{part_1}");
    println!("{part_2}");
}

fn process_part_2(input: &str) -> u32 {
    let grid = make_grid(input);

    let loop_positions = make_loop(&grid);

    let mut internal_positions = 0;

    for (y, row) in grid.iter().enumerate() {
        let mut is_internal = false;
        for (x, space) in row.iter().enumerate() {
            match space {
                Space::Empty => {
                    if is_internal {
                        internal_positions += 1;
                    }
                }
                Space::Pipe(pipe) => {
                    if loop_positions.contains(&Position::new(x as u32, y as u32)) {
                        if pipe.has_south_connection() {
                            is_internal = !is_internal;
                            continue;
                        }
                    } else if is_internal {
                        internal_positions += 1;
                    }
                }
            }
        }
    }

    internal_positions
}

fn process_part_1(input: &str) -> u32 {
    let grid = make_grid(input);

    let loop_positions = make_loop(&grid);

    (loop_positions.len() / 2).try_into().unwrap()
}

fn make_loop(grid: &Vec<Vec<Space>>) -> Vec<Position> {
    let start_position = find_start_position(grid).expect("Could not find start position");
    let mut visited_positions: Vec<Position> = vec![];

    let mut current_position = start_position;

    loop {
        visited_positions.push(current_position.clone());

        let move_candidates = get_valid_neighbours(&current_position, grid);

        if get_pipe_at_position(grid, move_candidates.get(0).unwrap()).is_start() {
            break;
        }

        let move_candidates: Vec<Position> = move_candidates
            .into_iter()
            .filter(|pos| !visited_positions.contains(pos))
            .collect();

        current_position = move_candidates.get(0).unwrap().clone();
    }

    visited_positions
}

fn get_valid_neighbours(centre_pos: &Position, grid: &Vec<Vec<Space>>) -> Vec<Position> {
    let current_pipe = get_pipe_at_position(grid, centre_pos);
    centre_pos
        .neighbours()
        .filter(|(candidate, direction)| {
            if !position_in_bounds(candidate, grid) {
                return false;
            }
            if let Space::Pipe(pipe) = &grid[candidate.y as usize][candidate.x as usize] {
                return current_pipe.connects_with(pipe, direction);
            }
            false
        })
        .map(|a| a.0)
        .collect()
}

fn position_in_bounds(position: &Position, grid: &Vec<Vec<Space>>) -> bool {
    let x = position.x;
    let y = position.y;

    if y > grid.len() as u32 - 1 {
        return false;
    }

    if x > grid[0].len() as u32 - 1 {
        return false;
    }

    true
}

fn get_pipe_at_position(grid: &[Vec<Space>], position: &Position) -> Pipe {
    if let Space::Pipe(pipe) = &grid[position.y as usize][position.x as usize] {
        return pipe.clone();
    }
    unreachable!()
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Clone)]
struct Pipe {
    n: bool,
    e: bool,
    s: bool,
    w: bool,
}

impl Pipe {
    fn new(n: bool, e: bool, s: bool, w: bool) -> Self {
        Self { n, e, s, w }
    }

    fn connects_with(&self, other: &Self, dir: &Direction) -> bool {
        match dir {
            Direction::North => self.n && other.s,
            Direction::East => self.e && other.w,
            Direction::South => self.s && other.n,
            Direction::West => self.w && other.e,
        }
    }

    fn is_start(&self) -> bool {
        self.n && self.e && self.s && self.w
    }

    fn has_south_connection(&self) -> bool {
        self.s
    }
}

impl TryFrom<&str> for Pipe {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "|" => Ok(Self::new(true, false, true, false)),
            "-" => Ok(Self::new(false, true, false, true)),
            "L" => Ok(Self::new(true, true, false, false)),
            "J" => Ok(Self::new(true, false, false, true)),
            "F" => Ok(Self::new(false, true, true, false)),
            "7" => Ok(Self::new(false, false, true, true)),
            "S" => Ok(Self::new(true, true, true, true)),
            other => Err(format!("Cannot convert {other} to Pipe")),
        }
    }
}

#[derive(Debug)]
enum Space {
    Pipe(Pipe),
    Empty,
}

impl From<&str> for Space {
    fn from(value: &str) -> Self {
        match value {
            "." => Self::Empty,
            a => match Pipe::try_from(a) {
                Ok(pipe) => Self::Pipe(pipe),
                _ => Self::Empty,
            },
        }
    }
}

fn make_grid(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|a| !a.is_empty())
                .map(|a| a.into())
                .collect()
        })
        .collect()
}

fn find_start_position(grid: &[Vec<Space>]) -> Option<Position> {
    for (y, row) in grid.iter().enumerate() {
        for (x, space) in row.iter().enumerate() {
            if let Space::Pipe(pipe) = space {
                if pipe.is_start() {
                    return Some(Position::new(x as u32, y as u32));
                }
            }
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn neighbours(&self) -> Neighbours {
        Neighbours::new(self.clone())
    }
}

struct Neighbours {
    neighbours: Vec<(Position, Direction)>,
    current_index: usize,
}

impl Neighbours {
    fn new(pos: Position) -> Self {
        let mut neighbours = vec![];

        if pos.y > 0 {
            neighbours.push((Position::new(pos.x, pos.y - 1), Direction::North));
        }

        neighbours.push((Position::new(pos.x + 1, pos.y), Direction::East));
        neighbours.push((Position::new(pos.x, pos.y + 1), Direction::South));

        if pos.x > 0 {
            neighbours.push((Position::new(pos.x - 1, pos.y), Direction::West));
        }

        Self {
            neighbours,
            current_index: 0,
        }
    }
}

impl Iterator for Neighbours {
    type Item = (Position, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.neighbours.len() {
            return None;
        }

        let next_pos = &self.neighbours[self.current_index];

        self.current_index += 1;

        Some(next_pos.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_a() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        let result = process_part_1(input);

        assert_eq!(result, 4);
    }

    #[test]
    fn part_1_b() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let result = process_part_1(input);

        assert_eq!(result, 8);
    }

    #[test]
    fn part_2_a() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let result = process_part_2(input);

        assert_eq!(result, 4);
    }

    #[test]
    fn part_2_b() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        let result = process_part_2(input);

        assert_eq!(result, 8);
    }
}
