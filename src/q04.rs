use crate::parser::*;
use std::io;

const INPUT_FILE: &str = "input/input04.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input04_sample.txt";

struct Input {
    elf_1: (u32, u32),
    elf_2: (u32, u32),
}

impl Input {
    fn range_fully_contained(&self) -> bool {
        let (s1, e1) = self.elf_1;
        let (s2, e2) = self.elf_2;
        (s1 <= s2 && e1 >= e2) || (s1 >= s2 && e1 <= e2)
    }

    fn has_any_overlap(&self) -> bool {
        let (s1, e1) = self.elf_1;
        let (s2, e2) = self.elf_2;
        (s1 <= s2 && s2 <= e1) || (s2 <= s1 && s1 <= e2)
    }
}

impl TryFrom<&mut FileLines> for Input {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let line = lines.next_result()?;
        let (elf_1, elf_2) = line.split_once(',').unwrap();
        let (elf_1_start, elf_1_end) = elf_1.split_once('-').unwrap();
        let (elf_2_start, elf_2_end) = elf_2.split_once('-').unwrap();
        Ok(Input {
            elf_1: (elf_1_start.parse().unwrap(), elf_1_end.parse().unwrap()),
            elf_2: (elf_2_start.parse().unwrap(), elf_2_end.parse().unwrap()),
        })
    }
}

impl Iterator for FileInput<FileLines, Input> {
    type Item = Input;

    fn next(&mut self) -> Option<Input> {
        parse_from(&mut self.source)
    }
}

pub fn part_1() -> io::Result<u32> {
    do_part_1(INPUT_FILE)
}

fn _part_1_sample() -> io::Result<u32> {
    do_part_1(_INPUT_FILE_SAMPLE)
}

fn do_part_1(input_file: &str) -> io::Result<u32> {
    let mut count = 0;
    for elf_pair in parse_lines::<Input>(input_file)? {
        if elf_pair.range_fully_contained() {
            count += 1;
        }
    }
    Ok(count)
}

pub fn part_2() -> io::Result<u32> {
    do_part_2(INPUT_FILE)
}

fn _part_2_sample() -> io::Result<u32> {
    do_part_2(_INPUT_FILE_SAMPLE)
}

fn do_part_2(input_file: &str) -> io::Result<u32> {
    let mut count = 0;
    for elf_pair in parse_lines::<Input>(input_file)? {
        if elf_pair.has_any_overlap() {
            count += 1;
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(580, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(2, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(895, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(4, _part_2_sample().unwrap());
    }
}
