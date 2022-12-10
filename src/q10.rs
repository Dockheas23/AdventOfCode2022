use crate::parser::*;
use std::io;

const INPUT_FILE: &str = "input/input10.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input10_sample.txt";

struct Clock(u16, i32);

impl Clock {
    fn tick(&mut self, x: i32) {
        self.0 += 1;
        if (self.0 + 20) % 40 == 0 && self.0 <= 220 {
            self.1 += i32::from(self.0) * x
        }
    }
}

enum Input {
    AddX(i32),
    Noop,
}

impl TryFrom<&mut FileLines> for Input {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let line = lines.next_result()?;
        let mut instruction = line.split(" ");
        match instruction.next() {
            Some("addx") => match instruction.next() {
                Some(i) => Ok(Input::AddX(i.parse().unwrap())),
                None => error("Failed to parse addx operand"),
            },
            Some("noop") => Ok(Input::Noop),
            _ => error("Failed to parse instruction"),
        }
    }
}

impl Iterator for FileInput<FileLines, Input> {
    type Item = Input;

    fn next(&mut self) -> Option<Input> {
        parse_from(&mut self.source)
    }
}

pub fn part_1() -> io::Result<i32> {
    do_part_1(INPUT_FILE)
}

fn _part_1_sample() -> io::Result<i32> {
    do_part_1(_INPUT_FILE_SAMPLE)
}

fn do_part_1(input_file: &str) -> io::Result<i32> {
    let mut instructions = parse_lines::<Input>(input_file)?;
    let mut x = 1;
    let mut clock = Clock(0, 0);
    let mut current_instruction = instructions.next();
    while clock.0 < 250 {
        let c = &current_instruction;
        match c {
            Some(Input::AddX(a)) => {
                clock.tick(x);
                clock.tick(x);
                x += a;
            }
            Some(Input::Noop) => clock.tick(x),
            None => break,
        }
        current_instruction = instructions.next();
    }
    Ok(clock.1)
}

pub fn part_2() -> io::Result<String> {
    do_part_2(INPUT_FILE)
}

fn _part_2_sample() -> io::Result<String> {
    do_part_2(_INPUT_FILE_SAMPLE)
}

fn do_part_2(input_file: &str) -> io::Result<String> {
    let mut instructions = parse_lines::<Input>(input_file)?;
    let mut x = 1;
    let mut clock = Clock(0, 0);
    let mut current_instruction = instructions.next();
    let mut output = String::from('\n');
    while clock.0 < 240 {
        let c = &current_instruction;
        match c {
            Some(Input::AddX(a)) => {
                clock.tick(x);
                draw_pixel(i32::from(clock.0), x, &mut output);
                clock.tick(x);
                draw_pixel(i32::from(clock.0), x, &mut output);
                x += a;
            }
            Some(Input::Noop) => {
                clock.tick(x);
                draw_pixel(i32::from(clock.0), x, &mut output);
            }
            None => break,
        }
        current_instruction = instructions.next();
    }
    Ok(output)
}

fn draw_pixel(current_cycle: i32, sprite: i32, output: &mut String) {
    let position = (current_cycle - 1) % 40;
    if sprite - 1 <= position && position <= sprite + 1 {
        output.push('#');
    } else {
        output.push('.');
    }
    if position == 39 {
        output.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(14540, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(13140, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        let mut s = String::from('\n');
        s += "####.#..#.####.####.####.#..#..##..####.\n";
        s += "#....#..#....#.#.......#.#..#.#..#....#.\n";
        s += "###..####...#..###....#..####.#......#..\n";
        s += "#....#..#..#...#.....#...#..#.#.....#...\n";
        s += "#....#..#.#....#....#....#..#.#..#.#....\n";
        s += "####.#..#.####.#....####.#..#..##..####.\n";
        assert_eq!(s, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        let mut s = String::from('\n');
        s += "##..##..##..##..##..##..##..##..##..##..\n";
        s += "###...###...###...###...###...###...###.\n";
        s += "####....####....####....####....####....\n";
        s += "#####.....#####.....#####.....#####.....\n";
        s += "######......######......######......####\n";
        s += "#######.......#######.......#######.....\n";
        assert_eq!(s, _part_2_sample().unwrap());
    }
}
