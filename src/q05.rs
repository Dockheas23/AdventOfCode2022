use crate::parser::*;
use std::io;

const INPUT_FILE: &str = "input/input05.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input05_sample.txt";

#[derive(Debug, PartialEq)]
struct Move(usize, usize, usize);

#[derive(Debug, PartialEq)]
struct Input {
    crates: Vec<Vec<char>>,
    moves: Vec<Move>,
}

impl TryFrom<&mut FileLines> for Input {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let mut crates: Vec<Vec<char>> = vec![vec![]; 9];
        let mut moves: Vec<Move> = vec![];
        while let Some(line) = lines.next() {
            if line == "" {
                break;
            }
            let mut stack = 0;
            let mut chars = line.chars();
            while let Some(c) = chars.next() {
                match c {
                    ' ' => {
                        chars.next();
                        chars.next();
                        chars.next();
                    }
                    '[' => {
                        let item = chars.next().unwrap();
                        chars.next();
                        chars.next();
                        crates[stack].insert(0, item);
                    }
                    _ => {}
                }
                stack += 1;
            }
        }
        for line in lines {
            let parts = line.split(' ').collect::<Vec<_>>();
            moves.push(Move(
                parts[1].parse::<usize>().unwrap(),
                parts[3].parse::<usize>().unwrap(),
                parts[5].parse::<usize>().unwrap(),
            ));
        }
        Ok(Input {
            crates: crates,
            moves: moves,
        })
    }
}

pub fn part_1() -> io::Result<String> {
    let input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_1(input))
}

fn _part_1_sample() -> io::Result<String> {
    let input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_1(input))
}

fn do_part_1(input: Input) -> String {
    let mut crates = input.crates;
    for m in &input.moves {
        let (count, from, to) = (m.0, m.1, m.2);
        for _ in 0..count {
            let item = crates[from - 1].pop().unwrap();
            crates[to - 1].push(item);
        }
    }
    let mut result = String::from("");
    for c in crates {
        if let Some(item) = c.last() {
            result += item.to_string().as_str();
        }
    }
    String::from(result)
}

pub fn part_2() -> io::Result<String> {
    let input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_2(input))
}

fn _part_2_sample() -> io::Result<String> {
    let input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_2(input))
}

fn do_part_2(input: Input) -> String {
    let mut crates = input.crates;
    for m in &input.moves {
        let (count, from, to) = (m.0, m.1, m.2);
        let from_crate = &mut crates[from - 1];
        let items = &mut from_crate.split_off(from_crate.len() - count);
        crates[to - 1].append(items);
    }
    let mut result = String::from("");
    for c in crates {
        if let Some(item) = c.last() {
            result += item.to_string().as_str();
        }
    }
    String::from(result)
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!("MQTPGLLDN", part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!("CMZ", _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!("LVZPSTTCZ", part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!("MCD", _part_2_sample().unwrap());
    }
}
