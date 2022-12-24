use crate::parser::*;
use std::io;

const INPUT_FILE: &str = "input/input25.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input25_sample.txt";

#[derive(Debug)]
struct Number(i64);

impl Number {
    fn from_decimal(n: i64) -> Self {
        Self(n)
    }

    fn from_snafu(mut s: String) -> Self {
        let mut total = 0;
        let mut digit = 1;
        while let Some(c) = s.pop() {
            let factor = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("invalid SNAFU character: {}", c),
            };
            total += digit * factor;
            digit *= 5;
        }
        Self(total)
    }

    fn as_decimal(&self) -> i64 {
        self.0
    }

    fn as_snafu(&self) -> String {
        let mut digits: Vec<char> = Vec::new();
        let mut number = self.0;
        println!("{}", number);
        while number != 0 {
            let c = match number % 5 {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => {
                    number += 5;
                    '='
                }
                4 => {
                    number += 5;
                    '-'
                }
                _ => panic!("invalid mod result: {}", number % 5),
            };
            digits.push(c);
            number /= 5;
        }
        digits.reverse();
        String::from_iter(digits.into_iter())
    }
}

impl TryFrom<&mut FileLines> for Number {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        Ok(Number::from_snafu(lines.next_result()?))
    }
}

impl Iterator for FileInput<FileLines, Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Number> {
        parse_from(&mut self.source)
    }
}

pub fn part_1() -> io::Result<String> {
    do_part_1(INPUT_FILE)
}

fn _part_1_sample() -> io::Result<String> {
    do_part_1(_INPUT_FILE_SAMPLE)
}

fn do_part_1(input_file: &str) -> io::Result<String> {
    let input = parse_lines::<Number>(input_file)?;
    let sum: i64 = input.map(|n| n.as_decimal()).sum();
    Ok(Number::from_decimal(sum).as_snafu())
}

pub fn part_2() -> io::Result<String> {
    Ok(String::from("Day 25 part two was a free star!"))
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, part_1, Number};

    #[test]
    fn test_parse_snafu() {
        assert_eq!(
            4890,
            Number::from_snafu(String::from("2=-1=0")).as_decimal()
        )
    }

    #[test]
    fn test_to_snafu() {
        assert_eq!("2=-1=0", Number::from_decimal(4890).as_snafu())
    }

    #[test]
    fn test_part_1() {
        assert_eq!("2-=0-=-2=111=220=100", part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!("2=-1=0", _part_1_sample().unwrap());
    }
}
