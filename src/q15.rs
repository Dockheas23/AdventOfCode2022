use crate::parser::*;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::io;

const INPUT_FILE: &str = "input/input15.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input15_sample.txt";

type Range = (i32, i32);
type Ranges = Vec<(i32, i32)>;

#[derive(Debug)]
struct Sensor {
    position: Range,
    beacon: Range,
}

impl Sensor {
    fn beacon_distance(&self) -> u32 {
        let (x1, y1) = self.position;
        let (x2, y2) = self.beacon;
        x1.abs_diff(x2) + y1.abs_diff(y2)
    }

    fn project_to_y(&self, y: i32) -> Option<(i32, i32, u32)> {
        let (x1, y1) = self.position;
        let span = self.beacon_distance() as i32 - y.abs_diff(y1) as i32;
        if span > 0 {
            Some((x1, y, span as u32))
        } else {
            None
        }
    }

    fn projection_range(&self, y: i32, limit: i32) -> Option<Range> {
        match self.project_to_y(y) {
            Some((x, _, d)) => {
                let (a, b) = (x - d as i32, x + d as i32);
                if b < 0 || a > limit {
                    None
                } else {
                    Some((max(0, a), min(limit, b)))
                }
            }
            None => None,
        }
    }
}

impl TryFrom<&mut FileLines> for Sensor {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let line = lines.next_result()?;
        let (sensor, beacon) = line
            .strip_prefix("Sensor at x=")
            .unwrap()
            .split_once(": closest beacon is at x=")
            .unwrap();
        let (sensor_x, sensor_y) = sensor.split_once(", y=").unwrap();
        let (beacon_x, beacon_y) = beacon.split_once(", y=").unwrap();
        Ok(Sensor {
            position: (sensor_x.parse().unwrap(), sensor_y.parse().unwrap()),
            beacon: (beacon_x.parse().unwrap(), beacon_y.parse().unwrap()),
        })
    }
}

impl Iterator for FileInput<FileLines, Sensor> {
    type Item = Sensor;

    fn next(&mut self) -> Option<Sensor> {
        parse_from(&mut self.source)
    }
}

pub fn part_1() -> io::Result<usize> {
    const ROW: i32 = 2_000_000;
    do_part_1(INPUT_FILE, ROW)
}

fn _part_1_sample() -> io::Result<usize> {
    const ROW: i32 = 10;
    do_part_1(_INPUT_FILE_SAMPLE, ROW)
}

fn do_part_1(input_file: &str, row: i32) -> io::Result<usize> {
    let sensors = parse_lines::<Sensor>(input_file)?;
    let mut no_beacon: HashSet<i32> = HashSet::new();
    let mut beacon: HashSet<i32> = HashSet::new();
    for sensor in sensors {
        if sensor.beacon.1 == row {
            beacon.insert(sensor.beacon.0);
        }
        match sensor.project_to_y(row) {
            Some((x, _, d)) => {
                for i in 0..d + 1 {
                    no_beacon.insert(x + i as i32);
                    no_beacon.insert(x - i as i32);
                }
            }
            None => {}
        }
    }
    Ok(no_beacon.difference(&beacon).count())
}

pub fn part_2() -> io::Result<u64> {
    const LIMIT: usize = 4_000_000;
    do_part_2(INPUT_FILE, LIMIT)
}

fn _part_2_sample() -> io::Result<u64> {
    const LIMIT: usize = 20;
    do_part_2(_INPUT_FILE_SAMPLE, LIMIT)
}

fn do_part_2(input_file: &str, limit: usize) -> io::Result<u64> {
    let sensors = parse_lines::<Sensor>(input_file)?.collect();
    for i in 0..limit + 1 {
        if let Some(j) = find_uncovered(&sensors, i, (0, limit as i32)) {
            return Ok(tuning_frequency((j, i as i32)));
        }
    }
    error("Failed to find beacon")
}

fn find_uncovered(sensors: &Vec<Sensor>, row: usize, range: Range) -> Option<i32> {
    let mut covered: Ranges = Vec::new();
    for sensor in sensors.iter() {
        match sensor.projection_range(row as i32, range.1) {
            Some(range) => covered = merge_ranges(covered, range),
            None => {}
        }
        let (bx, by) = sensor.beacon;
        if by as usize == row && bx >= range.0 && bx <= range.1 {
            covered = merge_ranges(covered, (sensor.beacon.0, sensor.beacon.0));
        }
        if covered.len() >= 1 && covered[0] == range {
            println!("Row {} is fully covered!", row);
            return None;
        }
    }
    println!("Sensor sweep finished! Covered = {:?}", covered);
    if covered.len() != 1 || covered[0] != range {
        match covered[0] {
            (0, x) => Some(x + 1),
            _ => Some(0),
        }
    } else {
        None
    }
}

fn merge_ranges(ranges: Ranges, range: Range) -> Ranges {
    let length = ranges.len();
    if length == 0 {
        return vec![range];
    }
    let (l1, r1) = range;
    let mut result = vec![];
    let mut insert_left: usize = 0;
    let mut insert_right: usize = 0;
    for i in 0..length {
        if l1 <= ranges[i].1 + 1 {
            insert_left = i;
            insert_right = i;
            while insert_right + 1 < length && r1 + 1 >= ranges[insert_right + 1].0 {
                insert_right += 1;
            }
            break;
        } else {
            insert_left = i + 1;
            insert_right = i + 1;
            result.push(ranges[i]);
        }
    }
    match (insert_left, insert_right) {
        (i, _) if i == length => result.push(range),
        (i, _) if r1 + 1 < ranges[i].0 => {
            result.push(range);
            result.push(ranges[i]);
        }
        (i, j) => {
            let a = min(l1, ranges[i].0);
            let b = max(r1, ranges[j].1);
            result.push((a, b));
        }
    }
    for i in insert_right + 1..length {
        result.push(ranges[i]);
    }
    result
}

fn tuning_frequency(beacon: Range) -> u64 {
    let (x, y) = beacon;
    x as u64 * 4_000_000 + y as u64
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, merge_ranges, part_1, part_2};

    #[test]
    fn test_merge_ranges() {
        // Add
        assert_eq!(vec![(2, 3)], merge_ranges(vec![], (2, 3)));

        // Span
        assert_eq!(vec![(0, 5)], merge_ranges(vec![(0, 5)], (3, 4)));
        assert_eq!(vec![(0, 5)], merge_ranges(vec![(3, 4)], (0, 5)));
        assert_eq!(vec![(1, 4)], merge_ranges(vec![(1, 4)], (3, 3)));
        assert_eq!(vec![(1, 4)], merge_ranges(vec![(3, 3)], (1, 4)));

        // Disjoint
        assert_eq!(vec![(0, 1), (3, 4)], merge_ranges(vec![(3, 4)], (0, 1)));
        assert_eq!(vec![(0, 1), (3, 4)], merge_ranges(vec![(0, 1)], (3, 4)));

        // Meet
        assert_eq!(vec![(0, 3)], merge_ranges(vec![(2, 3)], (0, 1)));
        assert_eq!(vec![(0, 3)], merge_ranges(vec![(0, 1)], (2, 3)));
        assert_eq!(vec![(1, 3)], merge_ranges(vec![(2, 3)], (1, 1)));
        assert_eq!(vec![(1, 3)], merge_ranges(vec![(1, 1)], (2, 3)));
        assert_eq!(vec![(1, 3)], merge_ranges(vec![(3, 3)], (1, 2)));
        assert_eq!(vec![(1, 3)], merge_ranges(vec![(1, 2)], (3, 3)));

        // Straddle
        assert_eq!(vec![(0, 20)], merge_ranges(vec![(0, 10)], (8, 20)));
        assert_eq!(vec![(0, 20)], merge_ranges(vec![(8, 20)], (0, 10)));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(5688618, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(26, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(12625383204261, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(56000011, _part_2_sample().unwrap());
    }
}
