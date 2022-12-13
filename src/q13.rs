use crate::parser::*;
use std::cmp;
use std::io;

const INPUT_FILE: &str = "input/input13.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input13_sample.txt";

#[derive(Debug)]
struct Packet {
    data: PacketData,
}

impl From<&str> for Packet {
    fn from(s: &str) -> Self {
        let data = &s[1..s.len() - 1];
        let mut stack: Vec<Vec<PacketData>> = Vec::new();
        let mut num_stack = String::new();
        stack.push(Vec::new());
        for c in data.chars() {
            match c {
                '[' => {
                    stack.push(Vec::new());
                }
                ',' => {
                    if !num_stack.is_empty() {
                        let number = PacketData::Int(num_stack.parse().unwrap());
                        stack.last_mut().unwrap().push(number);
                        num_stack.clear();
                    }
                }
                ']' => {
                    if !num_stack.is_empty() {
                        let number = PacketData::Int(num_stack.parse().unwrap());
                        stack.last_mut().unwrap().push(number);
                        num_stack.clear();
                    }
                    let packet = PacketData::List(stack.pop().unwrap());
                    stack.last_mut().unwrap().push(packet);
                }
                x if x.is_ascii_digit() => num_stack.push(x),
                _ => {}
            }
        }
        if !num_stack.is_empty() {
            let number = PacketData::Int(num_stack.parse().unwrap());
            stack.last_mut().unwrap().push(number);
            num_stack.clear();
        }
        Packet {
            data: PacketData::List(stack.pop().unwrap()),
        }
    }
}

#[derive(Debug)]
enum PacketData {
    Int(u32),
    List(Vec<PacketData>),
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PacketData::Int(l), PacketData::Int(r)) => l == r,
            (PacketData::List(l), PacketData::List(r)) => {
                l.len() == r.len() && l.iter().zip(r.iter()).all(|(x, y)| x == y)
            }
            (l, PacketData::Int(r)) => l == &PacketData::List(vec![PacketData::Int(*r)]),
            (PacketData::Int(l), r) => &PacketData::List(vec![PacketData::Int(*l)]) == r,
        }
    }
}

impl Eq for PacketData {}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (PacketData::Int(l), PacketData::Int(r)) => {
                return l.partial_cmp(r);
            }
            (PacketData::List(l), PacketData::List(r)) => {
                for (a, b) in l.iter().zip(r.iter()) {
                    let cmp = a.partial_cmp(b);
                    if cmp != Some(cmp::Ordering::Equal) {
                        return cmp;
                    }
                }
                if l.len() < r.len() {
                    return Some(cmp::Ordering::Less);
                } else if l.len() > r.len() {
                    return Some(cmp::Ordering::Greater);
                } else {
                    return Some(cmp::Ordering::Equal);
                }
            }
            (l, PacketData::Int(r)) => {
                return l.partial_cmp(&PacketData::List(vec![PacketData::Int(*r)]));
            }
            (PacketData::Int(l), r) => {
                return PacketData::List(vec![PacketData::Int(*l)]).partial_cmp(r);
            }
        }
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

impl TryFrom<&mut FileLines> for PacketPair {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let left = Packet::from(lines.next_result()?.as_str());
        let right = Packet::from(lines.next_result()?.as_str());
        lines.next();
        Ok(PacketPair { left, right })
    }
}

impl Iterator for FileInput<FileLines, PacketPair> {
    type Item = PacketPair;

    fn next(&mut self) -> Option<PacketPair> {
        parse_from(&mut self.source)
    }
}

impl TryFrom<&mut FileLines> for Packet {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        match lines.next_result()?.as_str() {
            "" => Packet::try_from(lines),
            x => Ok(Packet::from(x)),
        }
    }
}

impl Iterator for FileInput<FileLines, Packet> {
    type Item = Packet;

    fn next(&mut self) -> Option<Packet> {
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
    let packet_pairs = parse_lines::<PacketPair>(input_file)?;
    let mut total = 0;
    for (i, packet_pair) in packet_pairs.enumerate() {
        if packet_pair.left.data <= packet_pair.right.data {
            total += i + 1;
        }
    }
    Ok(total)
}

pub fn part_2() -> io::Result<usize> {
    do_part_2(INPUT_FILE)
}

fn _part_2_sample() -> io::Result<usize> {
    do_part_2(_INPUT_FILE_SAMPLE)
}

fn do_part_2(input_file: &str) -> io::Result<usize> {
    let mut packets: Vec<PacketData> = parse_lines::<Packet>(input_file)?.map(|p| p.data).collect();
    let mut total = 1;
    let divider_1 = PacketData::List(vec![PacketData::Int(2)]);
    let divider_2 = PacketData::List(vec![PacketData::Int(6)]);
    packets.push(divider_1);
    packets.push(divider_2);

    packets.sort();
    let divider_1 = PacketData::List(vec![PacketData::Int(2)]);
    let divider_2 = PacketData::List(vec![PacketData::Int(6)]);
    for (i, packet) in packets.iter().enumerate() {
        if packet == &divider_1 || packet == &divider_2 {
            total *= i + 1;
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(5720, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(13, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(23504, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(140, _part_2_sample().unwrap());
    }
}
