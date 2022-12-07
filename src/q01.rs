use crate::parser::FileLines;

const INPUT_FILE: &str = "input/input01.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input01_sample.txt";

pub fn part_1() -> std::io::Result<u32> {
    let elves = sort_by_calories(INPUT_FILE)?;
    Ok(elves[0])
}

fn _part_1_sample() -> std::io::Result<u32> {
    let elves = sort_by_calories(_INPUT_FILE_SAMPLE)?;
    Ok(elves[0])
}

pub fn part_2() -> std::io::Result<u32> {
    let elves = sort_by_calories(INPUT_FILE)?;
    Ok(elves[0] + elves[1] + elves[2])
}

fn _part_2_sample() -> std::io::Result<u32> {
    let elves = sort_by_calories(_INPUT_FILE_SAMPLE)?;
    Ok(elves[0] + elves[1] + elves[2])
}

fn sort_by_calories(input_file: &str) -> std::io::Result<Vec<u32>> {
    let mut calories: u32 = 0;
    let mut elves: Vec<u32> = Vec::new();

    for line in FileLines::new(input_file)? {
        match line.as_str() {
            "" => {
                elves.push(calories);
                calories = 0;
            }
            x => {
                calories += x.parse::<u32>().unwrap();
            }
        }
    }
    elves.push(calories);
    elves.sort_by(|a, b| b.cmp(a));
    Ok(elves)
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(68775, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(24000, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(202585, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(45000, _part_2_sample().unwrap());
    }
}
