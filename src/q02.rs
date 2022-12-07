use crate::parser::FileLines;
use std::io;

const INPUT_FILE: &str = "input/input02.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input02_sample.txt";

struct Round<'a> {
    opponent: &'a str,
    mine: &'a str,
}

impl<'a> Round<'a> {
    fn score_1(self) -> u32 {
        let item_score = match self.mine {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };
        let game_score = match (self.opponent, self.mine) {
            ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
            ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
            ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
            _ => 0,
        };
        item_score + game_score
    }

    fn score_2(self) -> u32 {
        let item_score = match (self.opponent, self.mine) {
            ("A", "Y") | ("B", "X") | ("C", "Z") => 1,
            ("A", "Z") | ("B", "Y") | ("C", "X") => 2,
            ("A", "X") | ("B", "Z") | ("C", "Y") => 3,
            _ => 0,
        };
        let game_score = match self.mine {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0,
        };
        item_score + game_score
    }
}

impl<'a> From<&'a str> for Round<'a> {
    fn from(s: &'a str) -> Self {
        let (x, y) = s.split_once(" ").unwrap();
        Round {
            opponent: x,
            mine: y,
        }
    }
}

pub fn part_1() -> io::Result<u32> {
    Ok(do_part_1(FileLines::new(INPUT_FILE)?))
}

fn _part_1_sample() -> io::Result<u32> {
    Ok(do_part_1(FileLines::new(_INPUT_FILE_SAMPLE)?))
}

fn do_part_1(lines: FileLines) -> u32 {
    let mut score = 0;
    for line in lines {
        score += Round::from(line.as_str()).score_1();
    }
    score
}

pub fn part_2() -> io::Result<u32> {
    Ok(do_part_2(FileLines::new(INPUT_FILE)?))
}

fn _part_2_sample() -> io::Result<u32> {
    Ok(do_part_2(FileLines::new(_INPUT_FILE_SAMPLE)?))
}

fn do_part_2(lines: FileLines) -> u32 {
    let mut score = 0;
    for line in lines {
        score += Round::from(line.as_str()).score_2();
    }
    score
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(15523, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(15, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(15702, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(12, _part_2_sample().unwrap());
    }
}
