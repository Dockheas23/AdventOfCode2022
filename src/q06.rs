use crate::parser::*;
use std::collections::HashMap;
use std::io;

const INPUT_FILE: &str = "input/input06.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input06_sample.txt";

pub fn part_1() -> io::Result<u32> {
    Ok(do_part_1(read_line(INPUT_FILE)?))
}

fn _part_1_sample() -> io::Result<u32> {
    Ok(do_part_1(read_line(_INPUT_FILE_SAMPLE)?))
}

fn do_part_1(line: String) -> u32 {
    let mut stream = line.into_bytes().into_iter();
    start_of_packet(&mut stream)
}

pub fn part_2() -> io::Result<u32> {
    Ok(do_part_2(read_line(INPUT_FILE)?))
}

fn _part_2_sample() -> io::Result<u32> {
    Ok(do_part_2(read_line(_INPUT_FILE_SAMPLE)?))
}

fn do_part_2(line: String) -> u32 {
    let mut stream = line.into_bytes().into_iter();
    start_of_message(&mut stream)
}

fn find_in_stream(stream: &mut impl Iterator<Item = u8>, distinct: usize) -> u32 {
    let mut count = 0;
    let mut buf: Vec<u8> = vec![];
    let mut char_counts: HashMap<u8, usize> = HashMap::new();
    while let Some(new) = stream.next() {
        count += 1;
        buf.insert(0, new);
        char_counts.insert(new, char_counts.get(&new).unwrap_or(&0) + 1);
        if buf.len() < distinct {
            continue;
        } else if buf.len() > distinct {
            let old = buf.pop().unwrap();
            char_counts.insert(old, char_counts.get(&old).unwrap_or(&0) - 1);
        }
        if buf
            .iter()
            .filter(|x| char_counts.get(x).unwrap_or(&0) != &1)
            .next()
            .is_none()
        {
            break;
        }
    }
    count
}

fn read_line(input_file: &str) -> io::Result<String> {
    let mut lines = FileLines::new(input_file)?;
    Ok(String::from(lines.next().unwrap()))
}

fn start_of_message(stream: &mut impl Iterator<Item = u8>) -> u32 {
    find_in_stream(stream, 14)
}

fn start_of_packet(stream: &mut impl Iterator<Item = u8>) -> u32 {
    find_in_stream(stream, 4)
}

#[cfg(test)]
mod tests {
    use super::{
        _part_1_sample, _part_2_sample, part_1, part_2, start_of_message, start_of_packet,
    };

    #[test]
    fn test_start_of_message() {
        let s1 = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(19, start_of_message(&mut s1.into_bytes().into_iter()));
        let s2 = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(23, start_of_message(&mut s2.into_bytes().into_iter()));
        let s3 = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(23, start_of_message(&mut s3.into_bytes().into_iter()));
        let s4 = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(29, start_of_message(&mut s4.into_bytes().into_iter()));
        let s5 = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(26, start_of_message(&mut s5.into_bytes().into_iter()));
    }

    #[test]
    fn test_start_of_packet() {
        let s1 = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(7, start_of_packet(&mut s1.into_bytes().into_iter()));
        let s2 = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(5, start_of_packet(&mut s2.into_bytes().into_iter()));
        let s3 = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(6, start_of_packet(&mut s3.into_bytes().into_iter()));
        let s4 = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(10, start_of_packet(&mut s4.into_bytes().into_iter()));
        let s5 = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(11, start_of_packet(&mut s5.into_bytes().into_iter()));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(1287, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(7, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3716, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(19, _part_2_sample().unwrap());
    }
}
