use crate::parser::*;
use std::io;

const INPUT_FILE: &str = "input/input20.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input20_sample.txt";

#[derive(Debug)]
struct Input {
    numbers: Vec<(usize, i64)>,
}

impl Input {
    fn scale(&mut self, scale: i64) {
        for i in 0..self.numbers.len() {
            self.numbers[i].1 = self.numbers[i].1 * scale;
        }
    }

    fn shift(&mut self, item_number: usize) {
        let index = self
            .numbers
            .iter()
            .position(|&(i, _)| i == item_number)
            .unwrap();
        let value = self.numbers[index].1;
        let mut new_index = (index as i64 + value) % (self.numbers.len() - 1) as i64;
        if new_index <= 0 {
            new_index += self.numbers.len() as i64 - 1;
        }
        self.numbers.remove(index);
        self.numbers
            .insert(new_index as usize, (item_number, value));
    }
}

impl TryFrom<&mut FileLines> for Input {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let mut numbers = Vec::new();
        for (i, line) in lines.enumerate() {
            numbers.push((i as usize, line.parse().unwrap()));
        }
        Ok(Input { numbers })
    }
}

pub fn part_1() -> io::Result<i64> {
    let mut input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_1(&mut input))
}

fn _part_1_sample() -> io::Result<i64> {
    let mut input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_1(&mut input))
}

fn do_part_1(input: &mut Input) -> i64 {
    let l = input.numbers.len();
    for i in 0..l {
        input.shift(i);
    }
    let i_zero = input.numbers.iter().position(|&(_, v)| v == 0).unwrap();
    input.numbers[(i_zero + 1000) % l].1
        + input.numbers[(i_zero + 2000) % l].1
        + input.numbers[(i_zero + 3000) % l].1
}

pub fn part_2() -> io::Result<i64> {
    let mut input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_2(&mut input))
}

fn _part_2_sample() -> io::Result<i64> {
    let mut input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_2(&mut input))
}

fn do_part_2(input: &mut Input) -> i64 {
    const DECRYPTION_KEY: i64 = 811589153;
    let l = input.numbers.len();
    input.scale(DECRYPTION_KEY);
    for _ in 0..10 {
        for i in 0..l {
            input.shift(i);
        }
    }
    let i_zero = input.numbers.iter().position(|&(_, v)| v == 0).unwrap();
    input.numbers[(i_zero + 1000) % l].1
        + input.numbers[(i_zero + 2000) % l].1
        + input.numbers[(i_zero + 3000) % l].1
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(4151, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(3, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(7848878698663, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(1623178306, _part_2_sample().unwrap());
    }
}
