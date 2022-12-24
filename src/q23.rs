use crate::parser::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io;

const INPUT_FILE: &str = "input/input23.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input23_sample.txt";
const OFFSET: usize = 100;

type Point = (usize, usize);

enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn starting_direction_order() -> VecDeque<Direction> {
        VecDeque::from([
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ])
    }
}

#[derive(Debug)]
struct Input {
    elves: HashSet<Point>,
}

impl TryFrom<&mut FileLines> for Input {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let mut elves = HashSet::new();
        for (i, line) in lines.enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    elves.insert((OFFSET + i, OFFSET + j));
                }
            }
        }
        Ok(Input { elves })
    }
}

pub fn part_1() -> io::Result<usize> {
    let input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_1(input, 10))
}

fn _part_1_sample() -> io::Result<usize> {
    let input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_1(input, 10))
}

fn do_part_1(input: Input, rounds: usize) -> usize {
    let mut elves = input.elves;
    let mut directions = Direction::starting_direction_order();
    for _ in 0..rounds {
        let proposals = proposed_new_positions(&elves, &directions);
        let mut new_positions = HashSet::new();
        for (destination, sources) in proposals {
            if sources.len() == 1 {
                new_positions.insert(destination);
            } else {
                for source in sources {
                    new_positions.insert(source);
                }
            }
        }
        elves = new_positions;
        let d = directions.pop_front().unwrap();
        directions.push_back(d);
    }
    calculate_area(&elves)
}

pub fn part_2() -> io::Result<usize> {
    let input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_2(input))
}

fn _part_2_sample() -> io::Result<usize> {
    let input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_2(input))
}

fn do_part_2(input: Input) -> usize {
    let mut elves = input.elves;
    let mut directions = Direction::starting_direction_order();
    let mut rounds = 0;
    loop {
        rounds += 1;
        let proposals = proposed_new_positions(&elves, &directions);
        if proposals
            .iter()
            .all(|(dst, src)| src.len() == 1 && &src[0] == dst)
        {
            break;
        }
        let mut new_positions = HashSet::new();
        for (destination, sources) in proposals {
            if sources.len() == 1 {
                new_positions.insert(destination);
            } else {
                for source in sources {
                    new_positions.insert(source);
                }
            }
        }
        elves = new_positions;
        let d = directions.pop_front().unwrap();
        directions.push_back(d);
    }
    rounds
}

fn proposed_new_positions(
    elves: &HashSet<Point>,
    directions: &VecDeque<Direction>,
) -> HashMap<Point, Vec<Point>> {
    let mut proposals = HashMap::new();
    for &(row, col) in elves {
        let nw = elves.get(&(row - 1, col - 1));
        let n = elves.get(&(row - 1, col));
        let ne = elves.get(&(row - 1, col + 1));
        let w = elves.get(&(row, col - 1));
        let e = elves.get(&(row, col + 1));
        let sw = elves.get(&(row + 1, col - 1));
        let s = elves.get(&(row + 1, col));
        let se = elves.get(&(row + 1, col + 1));
        if nw.or(n).or(ne).or(w).or(e).or(sw).or(s).or(se).is_none() {
            insert_or_append(&mut proposals, (row, col), (row, col));
        } else {
            let mut has_proposed: bool = false;
            for direction in directions {
                if has_proposed {
                    break;
                }
                match direction {
                    Direction::North => {
                        if nw.or(n).or(ne).is_none() {
                            insert_or_append(&mut proposals, (row - 1, col), (row, col));
                            has_proposed = true;
                        }
                    }
                    Direction::South => {
                        if sw.or(s).or(se).is_none() {
                            insert_or_append(&mut proposals, (row + 1, col), (row, col));
                            has_proposed = true;
                        }
                    }
                    Direction::West => {
                        if nw.or(w).or(sw).is_none() {
                            insert_or_append(&mut proposals, (row, col - 1), (row, col));
                            has_proposed = true;
                        }
                    }
                    Direction::East => {
                        if ne.or(e).or(se).is_none() {
                            insert_or_append(&mut proposals, (row, col + 1), (row, col));
                            has_proposed = true;
                        }
                    }
                }
            }
            if !has_proposed {
                insert_or_append(&mut proposals, (row, col), (row, col));
            }
        }
    }
    proposals
}

fn insert_or_append(proposals: &mut HashMap<Point, Vec<Point>>, destination: Point, source: Point) {
    if !proposals.contains_key(&destination) {
        proposals.insert(destination, vec![source]);
    } else {
        proposals.get_mut(&destination).unwrap().push(source);
    }
}

fn calculate_area(elves: &HashSet<Point>) -> usize {
    let min_row = elves.iter().map(|(r, _)| r).min().unwrap();
    let max_row = elves.iter().map(|(r, _)| r).max().unwrap();
    let min_col = elves.iter().map(|(_, c)| c).min().unwrap();
    let max_col = elves.iter().map(|(_, c)| c).max().unwrap();
    (max_row - min_row + 1) * (max_col - min_col + 1) - elves.len()
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(4005, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(110, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(1008, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(20, _part_2_sample().unwrap());
    }
}
