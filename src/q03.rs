use crate::parser::*;
use std::collections::HashSet;
use std::io;

const INPUT_FILE: &str = "input/input03.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input03_sample.txt";

struct Group {
    bag_1: HashSet<u8>,
    bag_2: HashSet<u8>,
    bag_3: HashSet<u8>,
}

impl TryFrom<&mut FileSplit> for Group {
    type Error = io::Error;

    fn try_from(lines: &mut FileSplit) -> Result<Self, Self::Error> {
        let l_1 = lines.next_result()?;
        let l_2 = lines.next_result()?;
        let l_3 = lines.next_result()?;
        Ok(Group {
            bag_1: HashSet::from_iter(l_1),
            bag_2: HashSet::from_iter(l_2),
            bag_3: HashSet::from_iter(l_3),
        })
    }
}

impl Iterator for FileInput<FileSplit, Group> {
    type Item = Group;

    fn next(&mut self) -> Option<Self::Item> {
        parse_from(&mut self.source)
    }
}

pub fn part_1() -> io::Result<u32> {
    Ok(do_part_1(FileLines::new(INPUT_FILE)?))
}

fn _part_1_sample() -> io::Result<u32> {
    Ok(do_part_1(FileLines::new(_INPUT_FILE_SAMPLE)?))
}

fn do_part_1(lines: FileLines) -> u32 {
    let mut sum = 0;
    for line in lines {
        let mut compartment_1 = line;
        let compartment_2 = compartment_1.split_off(compartment_1.len() / 2);
        let set_1: HashSet<&u8> = HashSet::from_iter(compartment_1.as_bytes());
        let set_2: HashSet<&u8> = HashSet::from_iter(compartment_2.as_bytes());
        for b in set_1 {
            if set_2.contains(b) {
                sum += convert_to_priority(b);
            }
        }
    }
    sum
}

pub fn part_2() -> Result<u32, io::Error> {
    do_part_2(INPUT_FILE)
}

fn _part_2_sample() -> Result<u32, io::Error> {
    do_part_2(_INPUT_FILE_SAMPLE)
}

fn do_part_2(input_file: &str) -> Result<u32, io::Error> {
    let mut sum = 0;
    for group in parse_bytes::<Group>(input_file, &b'\n')? {
        for item in group.bag_1 {
            if group.bag_2.contains(&item) && group.bag_3.contains(&item) {
                sum += convert_to_priority(&item);
            }
        }
    }
    Ok(sum)
}

fn convert_to_priority(byte: &u8) -> u32 {
    match *byte {
        x if x <= 90 => u32::from(x) - 38,
        x => u32::from(x) - 96,
    }
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, convert_to_priority, part_1, part_2};

    #[test]
    fn test_convert_to_priority() {
        assert_eq!(1, convert_to_priority(&97)); // a
        assert_eq!(26, convert_to_priority(&122)); // z
        assert_eq!(27, convert_to_priority(&65)); // A
        assert_eq!(52, convert_to_priority(&90)); // Z
        assert_eq!(16, convert_to_priority(&112)); // p
        assert_eq!(38, convert_to_priority(&76)); // L
        assert_eq!(42, convert_to_priority(&80)); // P
        assert_eq!(22, convert_to_priority(&118)); // v
        assert_eq!(20, convert_to_priority(&116)); // t
        assert_eq!(19, convert_to_priority(&115)); // s
    }

    #[test]
    fn test_part_1() {
        assert_eq!(8039, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(157, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(2510, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(70, _part_2_sample().unwrap());
    }
}
