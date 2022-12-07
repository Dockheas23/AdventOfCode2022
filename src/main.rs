mod parser;
mod q01;
mod q02;

fn print_result(number: u8, part1: &impl ToString, part2: &impl ToString) {
    println!("Question {:}", number);
    println!(
        "Part 1: {:}; Part 2: {:}",
        part1.to_string(),
        part2.to_string()
    );
    println!("------------");
}

fn main() -> std::io::Result<()> {
    println!("------------");
    print_result(1, &q01::part_1()?, &q01::part_2()?);
    print_result(2, &q02::part_1()?, &q02::part_2()?);
    Ok(())
}
