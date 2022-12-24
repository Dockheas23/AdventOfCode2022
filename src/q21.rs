use crate::parser::*;
use std::collections::HashMap;
use std::io;

const INPUT_FILE: &str = "input/input21.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input21_sample.txt";

#[derive(Debug)]
enum Operation {
    Plus(String, String),
    Minus(String, String),
    Multiply(String, String),
    Divide(String, String),
}

impl Operation {
    fn apply(&self, numbers: &HashMap<String, i64>) -> i64 {
        if !self.has_dependencies(numbers) {
            panic!();
        }
        match self {
            Operation::Plus(a, b) => numbers.get(a).unwrap() + numbers.get(b).unwrap(),
            Operation::Minus(a, b) => numbers.get(a).unwrap() - numbers.get(b).unwrap(),
            Operation::Multiply(a, b) => numbers.get(a).unwrap() * numbers.get(b).unwrap(),
            Operation::Divide(a, b) => numbers.get(a).unwrap() / numbers.get(b).unwrap(),
        }
    }

    fn has_dependencies(&self, numbers: &HashMap<String, i64>) -> bool {
        match self {
            Operation::Plus(a, b)
            | Operation::Minus(a, b)
            | Operation::Multiply(a, b)
            | Operation::Divide(a, b) => numbers.contains_key(a) && numbers.contains_key(b),
        }
    }
}

#[derive(Debug)]
struct Input {
    numbers: HashMap<String, i64>,
    operations: HashMap<String, Operation>,
}

impl TryFrom<&mut FileLines> for Input {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let mut numbers = HashMap::new();
        let mut operations = HashMap::new();
        for line in lines {
            let (monkey, action) = line.split_once(": ").unwrap();
            match action {
                a if a.contains("+") => {
                    let (m_a, m_b) = a.split_once(" + ").unwrap();
                    let operation = Operation::Plus(String::from(m_a), String::from(m_b));
                    operations.insert(String::from(monkey), operation);
                }
                a if a.contains("-") => {
                    let (m_a, m_b) = a.split_once(" - ").unwrap();
                    let operation = Operation::Minus(String::from(m_a), String::from(m_b));
                    operations.insert(String::from(monkey), operation);
                }
                a if a.contains("*") => {
                    let (m_a, m_b) = a.split_once(" * ").unwrap();
                    let operation = Operation::Multiply(String::from(m_a), String::from(m_b));
                    operations.insert(String::from(monkey), operation);
                }
                a if a.contains("/") => {
                    let (m_a, m_b) = a.split_once(" / ").unwrap();
                    let operation = Operation::Divide(String::from(m_a), String::from(m_b));
                    operations.insert(String::from(monkey), operation);
                }
                a => {
                    numbers.insert(String::from(monkey), a.parse().unwrap());
                }
            }
        }
        Ok(Input {
            numbers,
            operations,
        })
    }
}

pub fn part_1() -> io::Result<i64> {
    let mut input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_1(&mut input))
}

fn _part_1_sample() -> io::Result<i64> {
    let mut input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_1(&mut input))
}

fn do_part_1(input: &mut Input) -> i64 {
    let numbers = &mut input.numbers;
    let operations = &mut input.operations;
    while let Some((monkey, operation)) = operations
        .iter()
        .find(|(m, op)| !numbers.contains_key(&String::from(*m)) && op.has_dependencies(numbers))
    {
        let number = operation.apply(numbers);
        if monkey == "root" {
            return number;
        } else {
            let m = String::from(monkey);
            numbers.insert(m, number);
        }
    }
    panic!();
}

pub fn part_2() -> io::Result<i64> {
    let mut input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    input.numbers.remove("humn");
    Ok(do_part_2(&mut input))
}

fn _part_2_sample() -> io::Result<i64> {
    let mut input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    input.numbers.remove("humn");
    Ok(do_part_2(&mut input))
}

fn do_part_2(input: &mut Input) -> i64 {
    let numbers = &mut input.numbers;
    let operations = &mut input.operations;
    while let Some((monkey, operation)) = operations.iter().find(|(m, op)| {
        !numbers.contains_key(&String::from(*m))
            && op.has_dependencies(numbers)
            && m != &"root"
            && m != &"humn"
    }) {
        let number = operation.apply(numbers);
        let m = String::from(monkey);
        numbers.insert(m, number);
    }
    if let Some((_, operation)) = operations.iter().find(|(m, _)| m == &"root") {
        return match operation {
            Operation::Plus(a, b)
            | Operation::Minus(a, b)
            | Operation::Multiply(a, b)
            | Operation::Divide(a, b)
                if numbers.contains_key(a) =>
            {
                find_equality(
                    *numbers.get(a).unwrap(),
                    operations.get(b).unwrap(),
                    numbers,
                    operations,
                )
            }
            Operation::Plus(a, b)
            | Operation::Minus(a, b)
            | Operation::Multiply(a, b)
            | Operation::Divide(a, b) => find_equality(
                *numbers.get(b).unwrap(),
                operations.get(a).unwrap(),
                numbers,
                operations,
            ),
        };
    }
    panic!();
}

fn find_equality(
    value: i64,
    operation: &Operation,
    numbers: &HashMap<String, i64>,
    operations: &HashMap<String, Operation>,
) -> i64 {
    match operation {
        Operation::Plus(a, b) if a == "humn" => value - numbers.get(b).unwrap(),
        Operation::Plus(a, b) if b == "humn" => value - numbers.get(a).unwrap(),
        Operation::Plus(a, b) if numbers.contains_key(a) => find_equality(
            value - numbers.get(a).unwrap(),
            operations.get(b).unwrap(),
            numbers,
            operations,
        ),
        Operation::Plus(a, b) if numbers.contains_key(b) => find_equality(
            value - numbers.get(b).unwrap(),
            operations.get(a).unwrap(),
            numbers,
            operations,
        ),
        Operation::Minus(a, b) if a == "humn" => value + numbers.get(b).unwrap(),
        Operation::Minus(a, b) if b == "humn" => numbers.get(a).unwrap() - value,
        Operation::Minus(a, b) if numbers.contains_key(a) => find_equality(
            numbers.get(a).unwrap() - value,
            operations.get(b).unwrap(),
            numbers,
            operations,
        ),
        Operation::Minus(a, b) if numbers.contains_key(b) => find_equality(
            value + numbers.get(b).unwrap(),
            operations.get(a).unwrap(),
            numbers,
            operations,
        ),
        Operation::Multiply(a, b) if a == "humn" => value / numbers.get(b).unwrap(),
        Operation::Multiply(a, b) if b == "humn" => value / numbers.get(a).unwrap(),
        Operation::Multiply(a, b) if numbers.contains_key(a) => find_equality(
            value / numbers.get(a).unwrap(),
            operations.get(b).unwrap(),
            numbers,
            operations,
        ),
        Operation::Multiply(a, b) if numbers.contains_key(b) => find_equality(
            value / numbers.get(b).unwrap(),
            operations.get(a).unwrap(),
            numbers,
            operations,
        ),
        Operation::Divide(a, b) if a == "humn" => value * numbers.get(b).unwrap(),
        Operation::Divide(a, b) if b == "humn" => numbers.get(a).unwrap() / value,
        Operation::Divide(a, b) if numbers.contains_key(a) => find_equality(
            numbers.get(a).unwrap() / value,
            operations.get(b).unwrap(),
            numbers,
            operations,
        ),
        Operation::Divide(a, b) if numbers.contains_key(b) => find_equality(
            value * numbers.get(b).unwrap(),
            operations.get(a).unwrap(),
            numbers,
            operations,
        ),
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(223971851179174, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(152, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3379022190351, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(301, _part_2_sample().unwrap());
    }
}
