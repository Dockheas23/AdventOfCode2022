mod parser;

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
    Ok(())
}
