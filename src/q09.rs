use crate::parser::*;
use std::collections::HashSet;
use std::io;

const INPUT_FILE: &str = "input/input09.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input09_sample.txt";
const _INPUT_FILE_SAMPLE_2: &str = "input/input09_sample_2.txt";

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

struct Move(Direction, u32);

impl TryFrom<&mut FileLines> for Move {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        match lines.next_result()?.split_once(' ') {
            Some((direction, distance)) => match direction {
                "L" => Ok(Move(Direction::LEFT, distance.parse().unwrap())),
                "R" => Ok(Move(Direction::RIGHT, distance.parse().unwrap())),
                "U" => Ok(Move(Direction::UP, distance.parse().unwrap())),
                "D" => Ok(Move(Direction::DOWN, distance.parse().unwrap())),
                _ => error("Failed to parse direction"),
            },
            None => error("Failed to match line with direction and distance"),
        }
    }
}

impl Iterator for FileInput<FileLines, Move> {
    type Item = Move;

    fn next(&mut self) -> Option<Move> {
        parse_from(&mut self.source)
    }
}

pub fn part_1() -> io::Result<usize> {
    do_part_1(INPUT_FILE)
}

fn _part_1_sample() -> io::Result<usize> {
    do_part_1(_INPUT_FILE_SAMPLE)
}

fn do_part_1(input_file: &str) -> io::Result<usize> {
    let moves = parse_lines::<Move>(input_file)?;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    for moov in moves {
        let direction = moov.0;
        for _ in 0..moov.1 {
            match direction {
                Direction::LEFT => head.0 -= 1,
                Direction::RIGHT => head.0 += 1,
                Direction::UP => head.1 += 1,
                Direction::DOWN => head.1 -= 1,
            }
            follow_knot(&head, &mut tail);
            visited.insert(tail);
        }
    }
    Ok(visited.len())
}

pub fn part_2() -> io::Result<usize> {
    do_part_2(INPUT_FILE)
}

fn _part_2_sample() -> io::Result<usize> {
    do_part_2(_INPUT_FILE_SAMPLE)
}

fn _part_2_sample_2() -> io::Result<usize> {
    do_part_2(_INPUT_FILE_SAMPLE_2)
}

fn do_part_2(input_file: &str) -> io::Result<usize> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); 10];
    let moves = parse_lines::<Move>(input_file)?;
    for moov in moves {
        let direction = moov.0;
        for _ in 0..moov.1 {
            match direction {
                Direction::LEFT => knots[0].0 -= 1,
                Direction::RIGHT => knots[0].0 += 1,
                Direction::UP => knots[0].1 += 1,
                Direction::DOWN => knots[0].1 -= 1,
            }
            for i in 1..knots.len() {
                let head = knots[i - 1];
                let tail = &mut knots[i];
                follow_knot(&head, tail);
            }
            visited.insert(knots[9]);
        }
    }
    Ok(visited.len())
}

fn follow_knot(head: &(i32, i32), tail: &mut (i32, i32)) {
    let diff_x = tail.0 - head.0;
    let diff_y = tail.1 - head.1;

    if diff_x.abs() == 2 {
        tail.0 -= diff_x / 2;
        if diff_y.abs() == 1 {
            tail.1 = head.1;
        } else if diff_y == 2 {
            tail.1 -= 1;
        } else if diff_y == -2 {
            tail.1 += 1;
        }
    } else if diff_y.abs() == 2 {
        tail.1 -= diff_y / 2;
        if diff_x.abs() == 1 {
            tail.0 = head.0;
        } else if diff_x == 2 {
            tail.0 -= 1;
        } else if diff_x == -2 {
            tail.0 += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, _part_2_sample_2, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(6332, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(13, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(2511, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(1, _part_2_sample().unwrap());
    }

    #[test]
    fn test_part_2_sample_2() {
        assert_eq!(36, _part_2_sample_2().unwrap());
    }
}
