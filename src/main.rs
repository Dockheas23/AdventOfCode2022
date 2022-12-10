mod parser;
mod q01;
mod q02;
mod q03;
mod q04;
mod q05;
mod q06;
mod q07;
mod q08;
mod q09;
mod q10;

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
    print_result(3, &q03::part_1()?, &q03::part_2()?);
    print_result(4, &q04::part_1()?, &q04::part_2()?);
    print_result(5, &q05::part_1()?, &q05::part_2()?);
    print_result(6, &q06::part_1()?, &q06::part_2()?);
    print_result(7, &q07::part_1()?, &q07::part_2()?);
    print_result(8, &q08::part_1()?, &q08::part_2()?);
    print_result(9, &q09::part_1()?, &q09::part_2()?);
    print_result(10, &q10::part_1()?, &q10::part_2()?);
    Ok(())
}
