use crate::parser::*;
use std::collections::HashSet;
use std::io;

const INPUT_FILE: &str = "input/input14.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input14_sample.txt";
const SOURCE: Point = (500, 0);

type Point = (u32, u32);

#[derive(Debug)]
struct Input {
    grid: HashSet<Point>,
    abyss_depth: u32,
}

impl TryFrom<FileLines> for Input {
    type Error = io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut grid = HashSet::new();
        for line in lines {
            let mut points = line.split(" -> ");
            let mut last_point = convert_point(points.next().unwrap());
            for point in points {
                let current_point = convert_point(point);
                match (last_point, current_point) {
                    ((x1, y1), (x2, y2)) if x1 == x2 => {
                        let a = std::cmp::min(y1, y2);
                        let b = a + y1.abs_diff(y2);
                        for y in a..b {
                            grid.insert((x1, y));
                        }
                        grid.insert((x1, b));
                    }
                    ((x1, y1), (x2, y2)) if y1 == y2 => {
                        let a = std::cmp::min(x1, x2);
                        let b = a + x1.abs_diff(x2);
                        for x in a..b {
                            grid.insert((x, y1));
                        }
                        grid.insert((b, y1));
                    }
                    _ => error("Input does not appear vertical")?,
                }
                last_point = current_point;
            }
        }
        let abyss_depth = grid.iter().map(|p| p.1).max().unwrap();
        Ok(Input { grid, abyss_depth })
    }
}

pub fn part_1() -> io::Result<u32> {
    let input = Input::try_from(FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_1(input))
}

fn _part_1_sample() -> io::Result<u32> {
    let input = Input::try_from(FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_1(input))
}

fn do_part_1(mut input: Input) -> u32 {
    let mut grains = 0;

    while let Some(grain) = drop_grain_with_abyss(&input.grid, input.abyss_depth) {
        input.grid.insert(grain);
        grains += 1;
    }
    grains
}

pub fn part_2() -> io::Result<u32> {
    let input = Input::try_from(FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_2(input))
}

fn _part_2_sample() -> io::Result<u32> {
    let input = Input::try_from(FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_2(input))
}

fn do_part_2(mut input: Input) -> u32 {
    let mut grains = 0;
    let floor_depth = input.abyss_depth + 2;

    while let Some(grain) = drop_grain_with_floor(&input.grid, floor_depth) {
        input.grid.insert(grain);
        grains += 1;
    }
    grains
}

fn convert_point(string: &str) -> (u32, u32) {
    match string.split_once(",") {
        Some((x, y)) => (x.parse().unwrap(), y.parse().unwrap()),
        None => (0, 0),
    }
}

fn drop_grain_with_abyss(grid: &HashSet<Point>, abyss_depth: u32) -> Option<Point> {
    let (mut x, mut y) = SOURCE;
    while y < abyss_depth {
        if !grid.contains(&(x, y + 1)) {
            y += 1;
        } else if !grid.contains(&(x - 1, y + 1)) {
            x -= 1;
            y += 1;
        } else if !grid.contains(&(x + 1, y + 1)) {
            x += 1;
            y += 1;
        } else {
            return Some((x, y));
        }
    }
    None
}

fn drop_grain_with_floor(grid: &HashSet<Point>, floor_depth: u32) -> Option<Point> {
    let (mut x, mut y) = SOURCE;
    if grid.contains(&SOURCE) {
        return None;
    }
    loop {
        if !grid.contains(&(x, y + 1)) && y < floor_depth - 1 {
            y += 1;
        } else if !grid.contains(&(x - 1, y + 1)) && y < floor_depth - 1 {
            x -= 1;
            y += 1;
        } else if !grid.contains(&(x + 1, y + 1)) && y < floor_depth - 1 {
            x += 1;
            y += 1;
        } else {
            return Some((x, y));
        }
    }
}

fn _draw_grid(rocks: &HashSet<Point>, sand: &HashSet<Point>) {
    for y in 0..175 {
        print!("{} ", y);
        for x in 480..520 {
            let c = if sand.contains(&(x, y)) {
                'o'
            } else if rocks.contains(&(x, y)) {
                '#'
            } else if x == 500 {
                '+'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
}

fn _debug() -> io::Result<u32> {
    let mut input = Input::try_from(FileLines::new(INPUT_FILE)?)?;
    let mut grains = 0;
    let mut sand = HashSet::new();

    _draw_grid(&input.grid, &sand);
    println!();
    while let Some(grain) = drop_grain_with_abyss(&input.grid, input.abyss_depth) {
        sand.insert(grain);
        input.grid.insert(grain);
        grains += 1;
    }
    _draw_grid(&input.grid, &sand);
    Ok(grains)
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(799, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(24, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(29076, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(93, _part_2_sample().unwrap());
    }
}
