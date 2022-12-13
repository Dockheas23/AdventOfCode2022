use crate::parser::*;
use std::collections::{HashMap, HashSet};
use std::io;

const INPUT_FILE: &str = "input/input12.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input12_sample.txt";

type Map = Vec<Vec<PointInfo>>;
type Point = (usize, usize);

struct Input {
    map: Map,
}

impl TryFrom<FileLines> for Input {
    type Error = io::Error;

    fn try_from(lines: FileLines) -> Result<Self, Self::Error> {
        let mut map = Vec::new();
        for (_, line) in lines.enumerate() {
            let mut r = Vec::new();
            for (_, c) in line.bytes().enumerate() {
                r.push(PointInfo {
                    elevation: c,
                    f_score: usize::MAX,
                    g_score: usize::MAX,
                });
            }
            map.push(r);
        }
        Ok(Input { map })
    }
}

struct PointInfo {
    elevation: u8,
    f_score: usize,
    g_score: usize,
}

pub fn part_1() -> io::Result<usize> {
    let mut input = Input::try_from(FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_1(&mut input))
}

fn _part_1_sample() -> io::Result<usize> {
    let mut input = Input::try_from(FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_1(&mut input))
}

fn do_part_1(input: &mut Input) -> usize {
    let start = find_point(&input.map, b'S').unwrap();
    let end = find_point(&input.map, b'E').unwrap();
    a_star(start, end, heuristic, &mut input.map).unwrap()
}

pub fn part_2() -> io::Result<usize> {
    let mut input = Input::try_from(FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_2(&mut input))
}

fn _part_2_sample() -> io::Result<usize> {
    let mut input = Input::try_from(FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_2(&mut input))
}

fn do_part_2(input: &mut Input) -> usize {
    let mut start_points = find_all_at_elevation(&input.map, b'a');
    let end = find_point(&input.map, b'E').unwrap();

    start_points.push(find_point(&input.map, b'S').unwrap());
    let shortest = start_points
        .into_iter()
        .flat_map(
            |start| match a_star(start, end, heuristic, &mut input.map) {
                Some(distance) => vec![distance],
                None => vec![],
            },
        )
        .min()
        .unwrap();
    shortest
}

fn find_point(map: &Map, marker: u8) -> Option<Point> {
    for row in 0..map.len() {
        for column in 0..map[0].len() {
            if map[row][column].elevation == marker {
                return Some((row, column));
            }
        }
    }
    None
}

fn find_all_at_elevation(map: &Map, marker: u8) -> Vec<Point> {
    let mut result = vec![];
    for row in 0..map.len() {
        for column in 0..map[0].len() {
            if map[row][column].elevation == marker {
                result.push((row, column));
            }
        }
    }
    result
}

fn find_neighbours(point: Point, map: &Map) -> Vec<Point> {
    let mut result = vec![];
    let (row, column) = point;
    let e = map[row][column].elevation;
    if row > 0 && map[row - 1][column].elevation <= e + 1 {
        result.push((row - 1, column));
    }
    if row < map.len() - 1 && map[row + 1][column].elevation <= e + 1 {
        result.push((row + 1, column));
    }
    if column > 0 && map[row][column - 1].elevation <= e + 1 {
        result.push((row, column - 1));
    }
    if column < map[0].len() - 1 && map[row][column + 1].elevation <= e + 1 {
        result.push((row, column + 1));
    }
    result
}

fn heuristic(node: Point, goal: Point) -> usize {
    goal.0.abs_diff(node.0) + goal.1.abs_diff(node.1)
}

fn a_star(start: Point, goal: Point, h: fn(Point, Point) -> usize, map: &mut Map) -> Option<usize> {
    let mut open_set: HashSet<Point> = HashSet::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut current: Point;
    open_set.insert(start);
    let mut start_vertex = &mut map[start.0][start.1];
    start_vertex.elevation = b'a';
    start_vertex.f_score = h(start, goal);
    start_vertex.g_score = 0;
    let mut end_vertex = &mut map[goal.0][goal.1];
    end_vertex.elevation = b'z';

    while !open_set.is_empty() {
        current = *open_set
            .iter()
            .min_by_key(|(r, c)| map[*r][*c].f_score)
            .unwrap();
        let ref mut current_vertex = map[current.0][current.1];
        if current == goal {
            return Some(current_vertex.g_score);
        }
        open_set.remove(&current);

        for neighbour in find_neighbours(current, map) {
            let new_score = map[current.0][current.1].g_score + 1;
            let mut neighbour_vertex = &mut map[neighbour.0][neighbour.1];
            if new_score < neighbour_vertex.g_score {
                came_from.insert(neighbour, current);
                neighbour_vertex.g_score = new_score;
                neighbour_vertex.f_score = new_score + h(neighbour, goal);

                if !open_set.iter().any(|&x| x == neighbour) {
                    open_set.insert(neighbour);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(534, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(31, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(525, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(29, _part_2_sample().unwrap());
    }
}
