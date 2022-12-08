use crate::parser::*;
use std::io;

const INPUT_FILE: &str = "input/input08.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input08_sample.txt";

#[derive(Debug)]
struct Input {
    grid: Vec<Vec<u8>>,
}

impl Input {
    fn ranges(&self) -> (usize, usize, usize) {
        let (row_count, column_count) = (self.grid.len(), self.grid[0].len());
        (row_count, column_count, row_count * column_count)
    }

    fn count_visible(&self) -> u32 {
        let (row_count, column_count, tree_count) = self.ranges();
        let mut visible_count = 0;
        for tree in 0..tree_count {
            let (x, y) = (tree / row_count, tree % row_count);
            let height = &self.grid[x][y];
            let mut hidden_left = false;
            let mut hidden_right = false;
            let mut hidden_up = false;
            let mut hidden_down = false;
            for i in 0..x {
                if self.grid[i][y] >= *height {
                    hidden_left = true;
                    break;
                }
            }
            for i in (x + 1)..column_count {
                if self.grid[i][y] >= *height {
                    hidden_right = true;
                    break;
                }
            }
            for i in 0..y {
                if self.grid[x][i] >= *height {
                    hidden_up = true;
                    break;
                }
            }
            for i in (y + 1)..row_count {
                if self.grid[x][i] >= *height {
                    hidden_down = true;
                    break;
                }
            }
            if !(hidden_left && hidden_right && hidden_up && hidden_down) {
                visible_count += 1;
            }
        }
        visible_count
    }

    fn find_highest_scenic_score(&self) -> u32 {
        let (row_count, _, tree_count) = self.ranges();
        let mut highest_score = 0;
        for tree in 0..tree_count {
            let (x, y) = (tree / row_count, tree % row_count);
            let score = self.scenic_score(x, y);
            if score > highest_score {
                highest_score = score;
            }
        }
        highest_score
    }

    fn scenic_score(&self, x: usize, y: usize) -> u32 {
        let (row_count, column_count, _) = self.ranges();
        let height = &self.grid[x][y];
        let mut left_score = 0;
        let mut right_score = 0;
        let mut up_score = 0;
        let mut down_score = 0;
        for i in 0..x {
            left_score += 1;
            if height <= &self.grid[x - 1 - i][y] {
                break;
            }
        }
        for i in (x + 1)..column_count {
            right_score += 1;
            if height <= &self.grid[i][y] {
                break;
            }
        }
        for i in 0..y {
            up_score += 1;
            if height <= &self.grid[x][y - 1 - i] {
                break;
            }
        }
        for i in (y + 1)..row_count {
            down_score += 1;
            if height <= &self.grid[x][i] {
                break;
            }
        }
        left_score * right_score * up_score * down_score
    }
}

impl TryFrom<&mut FileLines> for Input {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let mut grid: Vec<Vec<u8>> = Vec::new();
        for line in lines {
            let mut row: Vec<u8> = Vec::new();
            for b in line.as_bytes() {
                row.push(*b - b'0');
            }
            grid.push(row);
        }
        Ok(Input { grid: grid })
    }
}

pub fn part_1() -> io::Result<u32> {
    let input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(input.count_visible())
}

fn _part_1_sample() -> io::Result<u32> {
    let input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(input.count_visible())
}

pub fn part_2() -> io::Result<u32> {
    let input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(input.find_highest_scenic_score())
}

fn _part_2_sample() -> io::Result<u32> {
    let input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(input.find_highest_scenic_score())
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(1854, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(21, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(527340, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(8, _part_2_sample().unwrap());
    }
}
