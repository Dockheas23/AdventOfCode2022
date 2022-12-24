use crate::parser::*;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

const INPUT_FILE: &str = "input/input18.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input18_sample.txt";

#[derive(Debug, Eq, Hash, PartialEq)]
struct Cube(u32, u32, u32);

impl Cube {
    fn exposed_faces(&self, all_cubes: &HashSet<Cube>) -> u32 {
        let (x1, y1, z1) = (self.0, self.1, self.2);
        let adjacent_cubes = all_cubes
            .iter()
            .filter(
                |Cube(x, y, z)| match (x1.abs_diff(*x), y1.abs_diff(*y), z1.abs_diff(*z)) {
                    (1, 0, 0) | (0, 1, 0) | (0, 0, 1) => true,
                    _ => false,
                },
            )
            .count();
        6 - adjacent_cubes as u32
    }

    fn neighbours(&self) -> Vec<Cube> {
        let (x, y, z) = (self.0, self.1, self.2);
        let mut result = vec![Cube(x + 1, y, z), Cube(x, y + 1, z), Cube(x, y, z + 1)];
        if x > 0 {
            result.push(Cube(x - 1, y, z))
        }
        if y > 0 {
            result.push(Cube(x, y - 1, z))
        }
        if z > 0 {
            result.push(Cube(x, y, z - 1))
        }
        result
    }
}

impl TryFrom<&mut FileLines> for Cube {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let line = lines.next_result()?;
        let mut split = line.split(",");
        Ok(Cube(
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        ))
    }
}

impl Iterator for FileInput<FileLines, Cube> {
    type Item = Cube;

    fn next(&mut self) -> Option<Cube> {
        parse_from(&mut self.source)
    }
}

pub fn part_1() -> io::Result<i32> {
    do_part_1(INPUT_FILE)
}

fn _part_1_sample() -> io::Result<i32> {
    do_part_1(_INPUT_FILE_SAMPLE)
}

fn do_part_1(input_file: &str) -> io::Result<i32> {
    let cubes: HashSet<Cube> = parse_lines::<Cube>(input_file)?.collect();
    Ok(cubes.iter().map(|c| c.exposed_faces(&cubes) as i32).sum())
}

pub fn part_2() -> io::Result<i32> {
    do_part_2(INPUT_FILE)
}

fn _part_2_sample() -> io::Result<i32> {
    do_part_2(_INPUT_FILE_SAMPLE)
}

fn do_part_2(input_file: &str) -> io::Result<i32> {
    let real_cubes: HashSet<Cube> = parse_lines::<Cube>(input_file)?.collect();
    let mut air_pockets: HashSet<Cube> = HashSet::new();
    let mut candidates = potential_air_pockets(&real_cubes);
    while let Some(candidate) = candidates.pop() {
        let mut current_pocket: HashSet<Cube> = HashSet::new();
        let Cube(x, y, z) = candidate;
        current_pocket.insert(Cube(x, y, z));
        if expand_pocket_from(Cube(x, y, z), &mut current_pocket, &real_cubes) {
            air_pockets = air_pockets
                .union(&current_pocket)
                .map(|&Cube(x, y, z)| Cube(x, y, z))
                .collect();
        }
        candidates = candidates
            .into_iter()
            .filter(|x| !current_pocket.contains(x))
            .collect();
    }
    let effective_cubes: HashSet<Cube> = real_cubes
        .union(&air_pockets)
        .map(|&Cube(x, y, z)| Cube(x, y, z))
        .collect();
    Ok(effective_cubes
        .iter()
        .map(|c| c.exposed_faces(&effective_cubes) as i32)
        .sum())
}

const MIN_X: u32 = 1;
const MIN_Y: u32 = 0;
const MIN_Z: u32 = 0;
const MAX_X: u32 = 21;
const MAX_Y: u32 = 21;
const MAX_Z: u32 = 21;

fn expand_pocket_from(cube: Cube, pocket: &mut HashSet<Cube>, real_cubes: &HashSet<Cube>) -> bool {
    let Cube(x, y, z) = cube;
    pocket.insert(Cube(x, y, z));
    let mut queue = VecDeque::new();
    queue.push_back(Cube(x, y, z));
    while let Some(c) = queue.pop_front() {
        if c.0 <= MIN_X
            || c.0 >= MAX_X
            || c.1 <= MIN_Y
            || c.1 >= MAX_Y
            || c.2 <= MIN_Z
            || c.2 >= MAX_Z
        {
            println!("Pocket rejected! {}", pocket.len());
            return false;
        }
        for neighbour in c.neighbours() {
            match neighbour {
                a if !real_cubes.contains(&a) && !pocket.contains(&a) => {
                    pocket.insert(Cube(a.0, a.1, a.2));
                    queue.push_back(a);
                }
                _ => {}
            }
        }
    }
    println!("Pocket found! {}", pocket.len());
    true
}

fn potential_air_pockets(real_cubes: &HashSet<Cube>) -> Vec<Cube> {
    let mut result = Vec::new();
    let min_x = *real_cubes.iter().map(|Cube(x, _, _)| x).min().unwrap();
    let max_x = *real_cubes.iter().map(|Cube(x, _, _)| x).max().unwrap();
    let min_y = *real_cubes.iter().map(|Cube(_, y, _)| y).min().unwrap();
    let max_y = *real_cubes.iter().map(|Cube(_, y, _)| y).max().unwrap();
    let min_z = *real_cubes.iter().map(|Cube(_, _, z)| z).min().unwrap();
    let max_z = *real_cubes.iter().map(|Cube(_, _, z)| z).max().unwrap();
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            for z in min_z..max_z + 1 {
                if !real_cubes.contains(&Cube(x, y, z)) {
                    result.push(Cube(x, y, z));
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2, Cube};
    use std::collections::HashSet;

    #[test]
    fn test_cube_exposed_faces() {
        let cubes = HashSet::from([Cube(1, 1, 1), Cube(1, 1, 2)]);
        for cube in &cubes {
            assert_eq!(5, cube.exposed_faces(&cubes));
        }
    }

    #[test]
    fn test_cube_exposed_faces_2() {
        let cubes = HashSet::from([Cube(1, 1, 1), Cube(1, 1, 2), Cube(1, 2, 1)]);

        assert_eq!(
            14,
            cubes.iter().map(|c| c.exposed_faces(&cubes) as i32).sum()
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(4332, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(64, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(2524, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(58, _part_2_sample().unwrap());
    }
}
