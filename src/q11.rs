use crate::parser::*;
use std::io;

const INPUT_FILE: &str = "input/input11.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input11_sample.txt";

#[derive(Debug)]
enum Operand {
    Symbol,
    Number(u64),
}

impl From<&str> for Operand {
    fn from(s: &str) -> Self {
        match s {
            "old" => Operand::Symbol,
            _ => Operand::Number(s.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Operand),
    Multiply(Operand),
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        match self {
            Operation::Add(Operand::Symbol) => old + old,
            Operation::Add(Operand::Number(x)) => old + x,
            Operation::Multiply(Operand::Symbol) => old * old,
            Operation::Multiply(Operand::Number(x)) => old * x,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    inspection_count: u64,
    operation: Operation,
    divisor: u64,
    next_if_true: usize,
    next_if_false: usize,
}

impl Monkey {
    fn blank() -> Self {
        Self {
            items: Vec::new(),
            inspection_count: 0,
            operation: Operation::Add(Operand::Number(0)),
            divisor: 1,
            next_if_true: 0,
            next_if_false: 0,
        }
    }
}

#[derive(Debug)]
struct Input {
    monkeys: Vec<Monkey>,
}

impl TryFrom<&mut FileLines> for Input {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let mut monkeys: Vec<Monkey> = Vec::new();
        let mut monkey: Monkey = Monkey::blank();
        for line in lines {
            let mut words = line.trim().split(' ');
            match words.next() {
                Some("Starting") => {
                    words.next();
                    for starting_item in words {
                        let item: u64 = starting_item.trim_end_matches(',').parse().unwrap();
                        monkey.items.push(item);
                    }
                }
                Some("Operation:") => {
                    words.next();
                    words.next();
                    words.next();
                    let op = words.next();
                    let operand = match words.next() {
                        Some(s) => Operand::from(s),
                        None => error("Failed to parse operand")?,
                    };
                    match op {
                        Some("+") => monkey.operation = Operation::Add(operand),
                        Some("*") => monkey.operation = Operation::Multiply(operand),
                        _ => error("Failed to parse operator")?,
                    };
                }
                Some("Test:") => {
                    words.next();
                    words.next();
                    monkey.divisor = words.next().unwrap().parse().unwrap();
                }
                Some("If") => match words.next() {
                    Some("true:") => {
                        words.next();
                        words.next();
                        words.next();
                        monkey.next_if_true = words.next().unwrap().parse().unwrap();
                    }
                    Some("false:") => {
                        words.next();
                        words.next();
                        words.next();
                        monkey.next_if_false = words.next().unwrap().parse().unwrap();
                        monkeys.push(monkey);
                        monkey = Monkey::blank();
                    }
                    _ => error("Failed to parse If condition")?,
                },
                _ => {}
            }
        }
        Ok(Input { monkeys: monkeys })
    }
}

pub fn part_1() -> io::Result<u64> {
    do_part_1(INPUT_FILE)
}

fn _part_1_sample() -> io::Result<u64> {
    do_part_1(_INPUT_FILE_SAMPLE)
}

fn do_part_1(input_file: &str) -> io::Result<u64> {
    let input = Input::try_from(&mut FileLines::new(input_file)?)?;
    let mut monkeys = input.monkeys;
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            process_items(&mut monkeys, i);
        }
    }
    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    let monkey_business = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .take(2)
        .product();
    Ok(monkey_business)
}

pub fn part_2() -> io::Result<u64> {
    do_part_2(INPUT_FILE)
}

fn _part_2_sample() -> io::Result<u64> {
    do_part_2(_INPUT_FILE_SAMPLE)
}

fn do_part_2(input_file: &str) -> io::Result<u64> {
    let input = Input::try_from(&mut FileLines::new(input_file)?)?;
    let mut monkeys = input.monkeys;
    let factor: u64 = monkeys.iter().map(|m| m.divisor).product();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            process_items_reduced_worry(&mut monkeys, i, factor);
        }
    }
    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    let monkey_business = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .take(2)
        .product();
    Ok(monkey_business)
}

fn process_items(monkeys: &mut Vec<Monkey>, i: usize) {
    for _ in 0..monkeys[i].items.len() {
        monkeys[i].inspection_count += 1;
        let item = monkeys[i].items.pop().unwrap();
        let operated = monkeys[i].operation.apply(item) / 3;
        if operated % monkeys[i].divisor == 0 {
            let next = monkeys[i].next_if_true;
            monkeys[next].items.push(operated);
        } else {
            let next = monkeys[i].next_if_false;
            monkeys[next].items.push(operated);
        }
    }
}

fn process_items_reduced_worry(monkeys: &mut Vec<Monkey>, i: usize, factor: u64) {
    for _ in 0..monkeys[i].items.len() {
        monkeys[i].inspection_count += 1;
        let item = monkeys[i].items.pop().unwrap();
        let operated = monkeys[i].operation.apply(item) % factor;
        if operated % monkeys[i].divisor == 0 {
            let next = monkeys[i].next_if_true;
            monkeys[next].items.push(operated);
        } else {
            let next = monkeys[i].next_if_false;
            monkeys[next].items.push(operated);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(57838, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(10605, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(15050382231, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(2713310158, _part_2_sample().unwrap());
    }
}
