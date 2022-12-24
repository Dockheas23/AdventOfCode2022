use crate::parser::*;
use std::io;

const INPUT_FILE: &str = "input/input17.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input17_sample.txt";
const CHAMBER_WIDTH: u32 = 7;
const CLEARANCE: u32 = 3;
const INITIAL_X_OFFSET: u32 = 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ShapeType {
    HorizontalBar,
    Plus,
    BackwardL,
    VerticalBar,
    Square,
}

struct Shape {
    point: (u32, u32),
    shape_type: ShapeType,
}

impl Shape {
    fn new(shape_number: u64, height: u32) -> Self {
        match shape_number % 5 {
            1 => Shape {
                point: (INITIAL_X_OFFSET, height),
                shape_type: ShapeType::HorizontalBar,
            },
            2 => Shape {
                point: (INITIAL_X_OFFSET, height + 2),
                shape_type: ShapeType::Plus,
            },
            3 => Shape {
                point: (INITIAL_X_OFFSET, height + 2),
                shape_type: ShapeType::BackwardL,
            },
            4 => Shape {
                point: (INITIAL_X_OFFSET, height + 3),
                shape_type: ShapeType::VerticalBar,
            },
            _ => Shape {
                point: (INITIAL_X_OFFSET, height + 1),
                shape_type: ShapeType::Square,
            },
        }
    }

    fn yield_points(&self) -> Vec<(u32, u32)> {
        let (x, y) = self.point;
        match self.shape_type {
            ShapeType::HorizontalBar => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            ShapeType::Plus => vec![
                (x + 1, y),
                (x, y - 1),
                (x + 1, y - 1),
                (x + 2, y - 1),
                (x + 1, y - 2),
            ],
            ShapeType::BackwardL => vec![
                (x + 2, y),
                (x + 2, y - 1),
                (x, y - 2),
                (x + 1, y - 2),
                (x + 2, y - 2),
            ],
            ShapeType::VerticalBar => vec![(x, y), (x, y - 1), (x, y - 2), (x, y - 3)],
            ShapeType::Square => vec![(x, y), (x + 1, y), (x, y - 1), (x + 1, y - 1)],
        }
    }

    fn shift_left(&mut self, chamber: &Chamber) -> bool {
        let points = self.yield_points();
        if points.iter().any(|(x, _)| *x == 0) {
            false
        } else {
            let shifted: Vec<(u32, u32)> = points.iter().map(|(x, y)| (*x - 1, *y)).collect();
            if shifted.iter().any(|p| chamber.points.contains(&p)) {
                false
            } else {
                self.point.0 -= 1;
                true
            }
        }
    }

    fn shift_right(&mut self, chamber: &Chamber) -> bool {
        let points = self.yield_points();
        if points.iter().any(|(x, _)| *x == CHAMBER_WIDTH - 1) {
            false
        } else {
            let shifted: Vec<(u32, u32)> = points.iter().map(|(x, y)| (*x + 1, *y)).collect();
            if shifted.iter().any(|p| chamber.points.contains(&p)) {
                false
            } else {
                self.point.0 += 1;
                true
            }
        }
    }

    fn shift_down(&mut self, chamber: &Chamber) -> bool {
        let points = self.yield_points();
        if points.iter().any(|(_, y)| *y == 0) {
            false
        } else {
            let shifted: Vec<(u32, u32)> = points.iter().map(|(x, y)| (*x, *y - 1)).collect();
            if shifted.iter().any(|p| chamber.points.contains(&p)) {
                false
            } else {
                self.point.1 -= 1;
                true
            }
        }
    }
}

struct Chamber {
    points: Vec<(u32, u32)>,
}

impl Chamber {
    fn max_height(&self) -> u32 {
        self.points.iter().map(|(_, y)| *y + 1).max().unwrap_or(0)
    }

    fn add_shape(&mut self, shape: &Shape) {
        self.points.extend_from_slice(&shape.yield_points()[..]);
    }

    fn _print(&self, shape: &Shape) {
        let top = self.max_height() + 6;
        for i in 0..top + 1 {
            let y = top - i;
            print!("|");
            for x in 0..7 {
                let c = if self.points.contains(&(x, y)) {
                    '#'
                } else if shape.yield_points().contains(&(x, y)) {
                    '@'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!("|");
        }
        println!("---------");
        println!();
    }

    fn _print_base(&self, shape: &Shape) {
        for i in 0..12 {
            print!("|");
            let y = self.max_height() + 4 - i;
            for x in 0..7 {
                let c = if self.points.contains(&(x, y)) {
                    '#'
                } else if shape.yield_points().contains(&(x, y)) {
                    '@'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!("|");
        }
    }
}

struct Input {
    instructions: Vec<char>,
}

impl TryFrom<&mut FileLines> for Input {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        Ok(Input {
            instructions: lines.next_result()?.chars().collect(),
        })
    }
}

pub fn part_1() -> io::Result<u32> {
    let chamber = simulate(2022, INPUT_FILE)?;
    Ok(chamber.max_height())
}

fn _part_1_sample() -> io::Result<u32> {
    let chamber = simulate(2022, _INPUT_FILE_SAMPLE)?;
    Ok(chamber.max_height())
}

pub fn part_2() -> io::Result<u64> {
    let (shape_count, added_height) = calculate_fixed_point(INPUT_FILE)?;

    do_part_2(INPUT_FILE, shape_count, added_height)
}

fn _part_2_sample() -> io::Result<u64> {
    let (shape_count, added_height) = calculate_fixed_point(_INPUT_FILE_SAMPLE)?;

    do_part_2(_INPUT_FILE_SAMPLE, shape_count, added_height)
}

fn do_part_2(
    input_file: &str,
    fixed_point_shape_count: u64,
    fixed_point_added_height: u64,
) -> io::Result<u64> {
    const TOTAL_SHAPES: u64 = 1_000_000_000_000;
    let repeated_rounds = TOTAL_SHAPES / fixed_point_shape_count;
    let remaining_shapes = TOTAL_SHAPES - repeated_rounds * fixed_point_shape_count;
    let added_height_from_repeats = repeated_rounds * fixed_point_added_height;

    let chamber = simulate(remaining_shapes, input_file)?;
    Ok(chamber.max_height() as u64 + added_height_from_repeats)
}

fn simulate(shape_count: u64, input_file: &str) -> io::Result<Chamber> {
    let input = Input::try_from(&mut FileLines::new(input_file)?)?;
    let mut instructions = input.instructions.iter();
    let mut chamber = Chamber { points: Vec::new() };
    for shape_number in 1..shape_count + 1 {
        let mut shape = Shape::new(shape_number, chamber.max_height() + CLEARANCE);
        loop {
            match instructions.next() {
                Some('<') => {
                    shape.shift_left(&chamber);
                }
                Some('>') => {
                    shape.shift_right(&chamber);
                }
                None => {
                    instructions = input.instructions.iter();
                    continue;
                }
                x => panic!("Unexpected instruction: {:?}, shape={}", x, shape_number),
            }
            if !shape.shift_down(&chamber) {
                chamber.add_shape(&shape);
                break;
            }
        }
    }
    Ok(chamber)
}

fn calculate_fixed_point(input_file: &str) -> io::Result<(u64, u64)> {
    const SHAPE_COUNT: u64 = 100000;
    let input = Input::try_from(&mut FileLines::new(input_file)?)?;
    let mut instructions = input.instructions.iter();
    let mut chamber = Chamber { points: Vec::new() };
    let mut rounds: Vec<(u64, u64, ShapeType)> = Vec::new();
    let mut last_round_shape_number = 0;
    let mut last_round_total_height = 0;
    for shape_number in 1..SHAPE_COUNT + 1 {
        let mut shape = Shape::new(shape_number, chamber.max_height() + CLEARANCE);
        loop {
            match instructions.next() {
                Some('<') => {
                    shape.shift_left(&chamber);
                }
                Some('>') => {
                    shape.shift_right(&chamber);
                }
                None => {
                    let round_shape_count = shape_number - last_round_shape_number;
                    let round_height = chamber.max_height() as u64 - last_round_total_height;
                    rounds.push((round_shape_count, round_height, shape.shape_type));
                    if let Some(pattern) = find_repeating_pattern(&rounds) {
                        println!("{:?}", pattern);
                        return Ok(pattern);
                    } else {
                        last_round_shape_number = shape_number;
                        last_round_total_height = chamber.max_height() as u64;
                        instructions = input.instructions.iter();
                        continue;
                    }
                }
                x => panic!("Unexpected instruction: {:?}, shape={}", x, shape_number),
            }
            if !shape.shift_down(&chamber) {
                chamber.add_shape(&shape);
                break;
            }
        }
    }
    error(format!("Did not find a fixed point after {} shapes", SHAPE_COUNT).as_str())
}

fn find_repeating_pattern(rounds: &Vec<(u64, u64, ShapeType)>) -> Option<(u64, u64)> {
    let mut i = 2;
    let mut found_match = false;
    while i <= rounds.len() / 2 {
        if rounds[rounds.len() - 1] == rounds[rounds.len() - i] {
            found_match = true;
            break;
        }
        i += 1;
    }
    if found_match {
        let mut stack: Vec<(u64, u64, ShapeType)> = Vec::new();
        for j in 0..i - 1 {
            if rounds[rounds.len() - 1 - j] != rounds[rounds.len() - i - j] {
                return None;
            } else {
                stack.push(rounds[rounds.len() - 1 - j]);
            }
        }
        println!("{:?}", stack);
        let shape_count = stack.iter().map(|(a, _, _)| a).sum();
        let height = stack.iter().map(|(_, b, _)| b).sum();
        return Some((shape_count, height));
    }
    None
}

fn _investigate_part_2_patterns(input_file: &str) -> io::Result<u64> {
    let input = Input::try_from(&mut FileLines::new(input_file)?)?;
    let mut instructions = input.instructions.iter();
    let mut chamber = Chamber { points: Vec::new() };
    let mut i_round = 1;
    let mut fresh = true;
    for shape_number in 1..100000 {
        let mut shape = Shape::new(shape_number, chamber.max_height() + CLEARANCE);
        loop {
            match instructions.next() {
                Some('<') => {
                    fresh = false;
                    shape.shift_left(&chamber);
                }
                Some('>') => {
                    fresh = false;
                    shape.shift_right(&chamber);
                }
                None => {
                    let s = match shape.shape_type {
                        ShapeType::HorizontalBar => 1,
                        ShapeType::Plus => 2,
                        ShapeType::BackwardL => 3,
                        ShapeType::VerticalBar => 4,
                        ShapeType::Square => 5,
                    };
                    println!(
                        "I: {}; F: {}; S: {}; T: {}; M: {}; B: ",
                        i_round,
                        fresh,
                        shape_number,
                        s,
                        chamber.max_height(),
                    );
                    chamber._print_base(&shape);
                    println!();
                    i_round += 1;
                    instructions = input.instructions.iter();
                    continue;
                }
                x => panic!("Unexpected instruction: {:?}, shape={}", x, shape_number),
            }
            if !shape.shift_down(&chamber) {
                chamber.add_shape(&shape);
                fresh = true;
                break;
            }
        }
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(3085, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(3068, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1535483870924, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(1514285714288, _part_2_sample().unwrap());
    }
}
