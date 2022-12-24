use crate::parser::*;
use std::collections::{HashMap, HashSet};
use std::io;

const INPUT_FILE: &str = "input/input24.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input24_sample.txt";

type Point = (usize, usize);

#[derive(Clone, Copy, Debug)]
enum Blizzard {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

impl Blizzard {
    fn advance(self, height: usize, width: usize) -> Blizzard {
        match self {
            Blizzard::Up((r, c)) => {
                let new_r = if r - 1 == 0 { height - 2 } else { r - 1 };
                Blizzard::Up((new_r, c))
            }
            Blizzard::Down((r, c)) => {
                let new_r = if r + 1 == height - 1 { 1 } else { r + 1 };
                Blizzard::Down((new_r, c))
            }
            Blizzard::Left((r, c)) => {
                let new_c = if c - 1 == 0 { width - 2 } else { c - 1 };
                Blizzard::Left((r, new_c))
            }
            Blizzard::Right((r, c)) => {
                let new_c = if c + 1 == width - 1 { 1 } else { c + 1 };
                Blizzard::Right((r, new_c))
            }
        }
    }

    fn to_point(&self) -> Point {
        match self {
            Blizzard::Up((r, c)) => (*r, *c),
            Blizzard::Down((r, c)) => (*r, *c),
            Blizzard::Left((r, c)) => (*r, *c),
            Blizzard::Right((r, c)) => (*r, *c),
        }
    }

    fn _to_point_with_char(&self) -> (Point, char) {
        match self {
            Blizzard::Up((r, c)) => ((*r, *c), '^'),
            Blizzard::Down((r, c)) => ((*r, *c), 'v'),
            Blizzard::Left((r, c)) => ((*r, *c), '<'),
            Blizzard::Right((r, c)) => ((*r, *c), '>'),
        }
    }
}

#[derive(Debug)]
struct Input {
    blizzards: Vec<Blizzard>,
    blizzard_points: HashSet<Point>,
    height: usize,
    width: usize,
}

impl Input {
    fn new(blizzards: Vec<Blizzard>, height: usize, width: usize) -> Self {
        let mut input = Input {
            blizzards,
            blizzard_points: HashSet::new(),
            height,
            width,
        };
        input.update_blizzard_points();
        input
    }

    fn start(&self) -> Point {
        (0, 1)
    }

    fn goal(&self) -> Point {
        (self.height - 1, self.width - 2)
    }

    fn is_available_space(&self, position: &Point) -> bool {
        let &(r, c) = position;
        r > 0
            && r < self.height - 1
            && c > 0
            && c < self.width - 1
            && !self.blizzard_points.contains(&(r, c))
    }

    fn available_moves(&self, position: &Point) -> HashSet<Point> {
        let mut available_moves = HashSet::new();
        let point_before_goal = (self.height - 2, self.width - 2);
        if position == &self.start() || position == &(1, 1) {
            available_moves.insert(self.start());
            available_moves.insert((1, 1));
        } else if position == &self.goal() || position == &point_before_goal {
            available_moves.insert(self.goal());
            available_moves.insert(point_before_goal);
        }
        if position == &self.start() || position == &self.goal() {
            return available_moves;
        }
        let &(r, c) = position;
        vec![(r, c), (r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
            .iter()
            .filter(|p| self.is_available_space(p))
            .for_each(|&p| {
                available_moves.insert(p);
            });
        available_moves
    }

    fn move_blizzards(&mut self) {
        for i in 0..self.blizzards.len() {
            let new_blizzard = self.blizzards[i].advance(self.height, self.width);
            self.blizzards[i] = new_blizzard;
        }
        self.update_blizzard_points();
    }

    fn update_blizzard_points(&mut self) {
        self.blizzard_points = self.blizzards.iter().map(|b| b.to_point()).collect();
    }
}

impl TryFrom<&mut FileLines> for Input {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let mut blizzards = Vec::new();
        let mut height = 0;
        let mut width = 0;
        for (i, line) in lines.enumerate() {
            height += 1;
            width = line.len();
            for (j, c) in line.chars().enumerate() {
                match c {
                    '^' => blizzards.push(Blizzard::Up((i, j))),
                    'v' => blizzards.push(Blizzard::Down((i, j))),
                    '<' => blizzards.push(Blizzard::Left((i, j))),
                    '>' => blizzards.push(Blizzard::Right((i, j))),
                    _ => {}
                }
            }
        }
        Ok(Input::new(blizzards, height, width))
    }
}

pub fn part_1() -> io::Result<u32> {
    let input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_1(input))
}

fn _part_1_sample() -> io::Result<u32> {
    let input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_1(input))
}

fn do_part_1(mut input: Input) -> u32 {
    let start = input.start();
    let end = input.goal();
    navigate(&mut input, &start, &end)
}

pub fn part_2() -> io::Result<u32> {
    let input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_2(input))
}

fn _part_2_sample() -> io::Result<u32> {
    let input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_2(input))
}

fn do_part_2(mut input: Input) -> u32 {
    let start = input.start();
    let end = input.goal();
    navigate(&mut input, &start, &end)
        + navigate(&mut input, &end, &start)
        + navigate(&mut input, &start, &end)
}

fn navigate(input: &mut Input, start: &Point, end: &Point) -> u32 {
    let mut steps = 0;
    let mut positions = HashSet::new();
    positions.insert(*start);
    while !positions.contains(end) {
        let mut new_positions = HashSet::new();
        input.move_blizzards();
        for position in positions {
            for p in input.available_moves(&position) {
                new_positions.insert(p);
            }
        }
        positions = new_positions;
        steps += 1;
    }
    steps
}

fn _print_map(input: &Input) {
    let blizzard_points: HashMap<Point, char> = input
        .blizzards
        .iter()
        .map(|b| b._to_point_with_char())
        .collect();

    for r in 0..input.height {
        for c in 0..input.width {
            let character = if r == 0 || r == input.height - 1 || c == 0 || c == input.width - 1 {
                '#'
            } else if blizzard_points.contains_key(&(r, c)) {
                *blizzard_points.get(&(r, c)).unwrap()
            } else {
                '.'
            };
            print!("{}", character);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(245, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(18, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(798, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(54, _part_2_sample().unwrap());
    }
}
