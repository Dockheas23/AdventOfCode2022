use crate::parser::*;
use std::io;

const _DEBUG: bool = true;
const INPUT_FILE: &str = "input/input22.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input22_sample.txt";

#[derive(Debug)]
enum Instruction {
    Turn(LeftOrRight),
    Advance(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LeftOrRight {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Location {
    point: (usize, usize),
    direction: Direction,
}

impl Location {
    fn new(row: usize, column: usize, direction: Direction) -> Self {
        Self {
            point: (row, column),
            direction,
        }
    }

    fn password(&self) -> usize {
        let (row_index, column_index) = self.point;
        let d = match self.direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };
        1000 * (row_index + 1) + 4 * (column_index + 1) + d
    }

    fn new_direction(&self, left_or_right: LeftOrRight) -> Direction {
        match self.direction {
            Direction::Up => {
                if left_or_right == LeftOrRight::Left {
                    Direction::Left
                } else {
                    Direction::Right
                }
            }
            Direction::Down => {
                if left_or_right == LeftOrRight::Left {
                    Direction::Right
                } else {
                    Direction::Left
                }
            }
            Direction::Left => {
                if left_or_right == LeftOrRight::Left {
                    Direction::Down
                } else {
                    Direction::Up
                }
            }
            Direction::Right => {
                if left_or_right == LeftOrRight::Left {
                    Direction::Up
                } else {
                    Direction::Down
                }
            }
        }
    }

    fn _current_zone_sample(&self) -> u8 {
        let (row, column) = self.point;
        if row >= 12 || column >= 16 {
            panic!("Invalid row or column: {}, {}", row, column);
        }
        if row < 4 {
            1
        } else if row < 8 {
            if column < 4 {
                2
            } else if column < 8 {
                3
            } else {
                4
            }
        } else {
            if column < 12 {
                5
            } else {
                6
            }
        }
    }

    fn current_zone(&self) -> u8 {
        let (row, column) = self.point;
        if row >= 200 || column >= 150 {
            panic!("Invalid row or column: {}, {}", row, column);
        }
        if row < 50 {
            if column < 100 {
                1
            } else {
                2
            }
        } else if row < 100 {
            3
        } else if row < 150 {
            if column < 50 {
                4
            } else {
                5
            }
        } else {
            6
        }
    }

    fn _step_off_edge_sample(&self) -> Location {
        let (row, column) = self.point;
        match (self._current_zone_sample(), self.direction) {
            (1, Direction::Up) => {
                // zone 2, down
                Location::new(4, 11 - column, Direction::Up)
            }
            (1, Direction::Right) => {
                // zone 6, left, last column, inverse row
                Location::new(11 - row, 15, Direction::Left)
            }
            (1, Direction::Left) => {
                // zone 3, down
                Location::new(4, 4 + row, Direction::Down)
            }
            (2, Direction::Left) => {
                // zone 6, up
                Location::new(11, 19 - row, Direction::Up)
            }
            (2, Direction::Up) => {
                // zone 1, down
                Location::new(0, 11 - column, Direction::Down)
            }
            (2, Direction::Down) => {
                // zone 5, up
                Location::new(11, 11 - column, Direction::Up)
            }
            (3, Direction::Up) => {
                // zone 1, right
                Location::new(column - 4, 8, Direction::Right)
            }
            (3, Direction::Down) => {
                // zone 5, right
                Location::new(column + 4, 8, Direction::Right)
            }
            (4, Direction::Right) => {
                // zone 6, down
                Location::new(8, 19 - row, Direction::Down)
            }
            (5, Direction::Left) => {
                // zone 3, up
                Location::new(7, 15 - row, Direction::Up)
            }
            (5, Direction::Down) => {
                // zone 2, up
                Location::new(7, 11 - column, Direction::Up)
            }
            (6, Direction::Up) => {
                // zone 4, left
                Location::new(19 - column, 11, Direction::Left)
            }
            (6, Direction::Down) => {
                // zone 2, right
                Location::new(19 - column, 0, Direction::Right)
            }
            (6, Direction::Right) => {
                // zone 1, left
                Location::new(1, 11, Direction::Left)
            }
            _ => panic!("{:?}", self),
        }
    }

    fn step_off_edge_real(&self) -> Location {
        let (row, column) = self.point;
        match (self.current_zone(), self.direction) {
            (1, Direction::Up) => {
                // zone 6, right
                Location::new(100 + column, 0, Direction::Right)
            }
            (1, Direction::Left) => {
                // zone 4, right
                Location::new(149 - row, 0, Direction::Right)
            }
            (2, Direction::Up) => {
                // zone 6, up
                Location::new(199, column - 100, Direction::Up)
            }
            (2, Direction::Down) => {
                // zone 3, left
                Location::new(column - 50, 99, Direction::Left)
            }
            (2, Direction::Right) => {
                // zone 5, left
                Location::new(149 - row, 99, Direction::Left)
            }
            (3, Direction::Left) => {
                // zone 4, down
                Location::new(100, row - 50, Direction::Down)
            }
            (3, Direction::Right) => {
                // zone 2, up
                Location::new(49, row + 50, Direction::Up)
            }
            (4, Direction::Up) => {
                // zone 3, right
                Location::new(column + 50, 50, Direction::Right)
            }
            (4, Direction::Left) => {
                // zone 1, right
                Location::new(149 - row, 50, Direction::Right)
            }
            (5, Direction::Down) => {
                // zone 6, left
                Location::new(column + 100, 49, Direction::Left)
            }
            (5, Direction::Right) => {
                // zone 2, left
                Location::new(149 - row, 149, Direction::Left)
            }
            (6, Direction::Down) => {
                // zone 2, down
                Location::new(0, column + 100, Direction::Down)
            }
            (6, Direction::Left) => {
                // zone 1, down
                Location::new(0, row - 100, Direction::Down)
            }
            (6, Direction::Right) => {
                // zone 5, up
                Location::new(149, row - 100, Direction::Up)
            }
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Slot {
    Empty,
    Space,
    Wall,
}

#[derive(Debug)]
struct Map {
    points: Vec<Vec<Slot>>,
}

impl Map {
    fn start_location(&self) -> Location {
        let points = &self.points;
        let c = (0..points[0].len())
            .find(|&p| points[0][p] == Slot::Space)
            .unwrap();
        Location {
            point: (0, c),
            direction: Direction::Right,
        }
    }

    fn point(&self, location: &Location) -> Slot {
        self.points[location.point.0][location.point.1]
    }
}

#[derive(Debug)]
struct Input {
    map: Map,
    instructions: Vec<Instruction>,
}

impl TryFrom<Vec<String>> for Map {
    type Error = io::Error;

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        let mut points = Vec::new();
        for line in lines {
            let mut map_row = Vec::new();
            for c in line.chars() {
                match c {
                    ' ' => map_row.push(Slot::Empty),
                    '.' => map_row.push(Slot::Space),
                    '#' => map_row.push(Slot::Wall),
                    _ => error("Not a valid map character")?,
                }
            }
            for _ in map_row.len()..150 {
                map_row.push(Slot::Empty);
            }
            points.push(map_row);
        }
        Ok(Map { points })
    }
}

impl TryFrom<&mut FileLines> for Input {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let mut map_lines = Vec::new();
        let mut line = lines.next().unwrap();
        while line != "" {
            map_lines.push(line);
            line = lines.next().unwrap();
        }
        let map = Map::try_from(map_lines)?;
        let mut instructions = Vec::new();
        line = lines.next().unwrap();
        let mut chars = line.chars();
        let mut num_buffer = String::from("");
        while let Some(c) = chars.next() {
            match c {
                'L' | 'R' => {
                    if !num_buffer.is_empty() {
                        instructions.push(Instruction::Advance(num_buffer.parse().unwrap()));
                        num_buffer.clear();
                    }
                    let direction = match c {
                        'L' => LeftOrRight::Left,
                        'R' => LeftOrRight::Right,
                        _ => error("Not a direction (should not happen)")?,
                    };
                    instructions.push(Instruction::Turn(direction));
                }
                n if n.is_ascii_digit() => num_buffer.push(n),
                _ => error("Instruction is neither a digit nor a direction")?,
            }
        }
        if !num_buffer.is_empty() {
            instructions.push(Instruction::Advance(num_buffer.parse().unwrap()));
            num_buffer.clear();
        }
        Ok(Input { map, instructions })
    }
}

pub fn part_1() -> io::Result<usize> {
    let mut input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_1(&mut input))
}

fn _part_1_sample() -> io::Result<usize> {
    let mut input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_1(&mut input))
}

fn do_part_1(input: &mut Input) -> usize {
    let mut location = input.map.start_location();
    for instruction in &input.instructions {
        location = apply_instruction(&location, &input.map, instruction);
    }
    location.password()
}

pub fn part_2() -> io::Result<usize> {
    let mut input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_2(&mut input, Location::step_off_edge_real))
}

fn _part_2_sample() -> io::Result<usize> {
    let mut input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_2(&mut input, Location::_step_off_edge_sample))
}

fn do_part_2(input: &mut Input, step_off_edge: fn(&Location) -> Location) -> usize {
    let mut location = input.map.start_location();
    for instruction in &input.instructions {
        location = apply_instruction_on_cube(&location, &input.map, instruction, step_off_edge);
    }
    if _DEBUG {
        println!("Final location: {:?}", location);
    }
    location.password()
}

fn _debug_input(input: &Input) {
    if _DEBUG {
        println!("{:?}", input);
    }
}

fn _debug_location(location: &Location) {
    if _DEBUG {
        println!("{:?}", location);
    }
}

fn update_position(location: &Location, map: &Map, distance: usize) -> (usize, usize) {
    let (mut new_row, mut new_column) = location.point;
    for _ in 0..distance {
        let (row, column) = (new_row, new_column);
        match location.direction {
            Direction::Up => {
                let next_index = if row == 0 || map.points[row - 1][column] == Slot::Empty {
                    map.points.len()
                        - 1
                        - (0..map.points.len())
                            .find(|&i| map.points[map.points.len() - 1 - i][column] != Slot::Empty)
                            .unwrap()
                } else {
                    row - 1
                };
                if map.points[next_index][column] == Slot::Wall {
                    break;
                } else {
                    new_row = next_index;
                }
            }
            Direction::Down => {
                let next_index =
                    if row == map.points.len() - 1 || map.points[row + 1][column] == Slot::Empty {
                        (0..map.points.len())
                            .find(|&i| map.points[i][column] != Slot::Empty)
                            .unwrap()
                    } else {
                        row + 1
                    };
                if map.points[next_index][column] == Slot::Wall {
                    break;
                } else {
                    new_row = next_index;
                }
            }
            Direction::Left => {
                let next_index = if column == 0 || map.points[row][column - 1] == Slot::Empty {
                    map.points[row].len()
                        - 1
                        - (0..map.points[row].len())
                            .find(|&i| {
                                map.points[row][map.points[row].len() - 1 - i] != Slot::Empty
                            })
                            .unwrap()
                } else {
                    column - 1
                };
                if map.points[row][next_index] == Slot::Wall {
                    break;
                } else {
                    new_column = next_index;
                }
            }
            Direction::Right => {
                let next_index = if column == map.points[row].len() - 1
                    || map.points[row][column + 1] == Slot::Empty
                {
                    (0..map.points[row].len())
                        .find(|&i| map.points[row][i] != Slot::Empty)
                        .unwrap()
                } else {
                    column + 1
                };
                if map.points[row][next_index] == Slot::Wall {
                    break;
                } else {
                    new_column = next_index;
                }
            }
        }
    }
    (new_row, new_column)
}

fn update_location_on_cube(
    location: &Location,
    map: &Map,
    distance: usize,
    step_off_edge: fn(&Location) -> Location,
) -> Location {
    let mut current_location = *location;
    let mut next_location: Location;
    for _ in 0..distance {
        let (row, column) = current_location.point;
        match current_location.direction {
            Direction::Up => {
                next_location = if row == 0 || map.points[row - 1][column] == Slot::Empty {
                    step_off_edge(&current_location)
                } else {
                    Location {
                        point: (row - 1, column),
                        direction: current_location.direction,
                    }
                }
            }
            Direction::Down => {
                next_location =
                    if row == map.points.len() - 1 || map.points[row + 1][column] == Slot::Empty {
                        step_off_edge(&current_location)
                    } else {
                        Location {
                            point: (row + 1, column),
                            direction: current_location.direction,
                        }
                    }
            }
            Direction::Left => {
                next_location = if column == 0 || map.points[row][column - 1] == Slot::Empty {
                    step_off_edge(&current_location)
                } else {
                    Location {
                        point: (row, column - 1),
                        direction: current_location.direction,
                    }
                }
            }
            Direction::Right => {
                next_location = if column == map.points[row].len() - 1
                    || map.points[row][column + 1] == Slot::Empty
                {
                    step_off_edge(&current_location)
                } else {
                    Location {
                        point: (row, column + 1),
                        direction: current_location.direction,
                    }
                }
            }
        }
        if map.point(&next_location) == Slot::Wall {
            return current_location;
        } else {
            current_location = next_location;
        }
    }
    current_location
}

fn apply_instruction(location: &Location, map: &Map, instruction: &Instruction) -> Location {
    let (mut row, mut column) = location.point;
    let mut direction = location.direction;
    match instruction {
        Instruction::Advance(distance) => (row, column) = update_position(location, map, *distance),
        Instruction::Turn(d) => direction = location.new_direction(*d),
    }
    Location {
        point: (row, column),
        direction: direction,
    }
}

fn apply_instruction_on_cube(
    location: &Location,
    map: &Map,
    instruction: &Instruction,
    step_off_edge: fn(&Location) -> Location,
) -> Location {
    if _DEBUG {
        println!("Location: {:?}", location);
        println!("Instruction: {:?}", instruction);
    }
    match instruction {
        Instruction::Advance(distance) => {
            update_location_on_cube(location, map, *distance, step_off_edge)
        }
        Instruction::Turn(d) => Location {
            point: location.point,
            direction: location.new_direction(*d),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(57350, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(6032, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(104385, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(5031, _part_2_sample().unwrap());
    }
}
