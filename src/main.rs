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
mod q11;
mod q12;
mod q13;
mod q14;
mod q15;
mod q16;
mod q17;
mod q18;
mod q19;
mod q20;
mod q21;

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
    print_result(11, &q11::part_1()?, &q11::part_2()?);
    print_result(12, &q12::part_1()?, &q12::part_2()?);
    print_result(13, &q13::part_1()?, &q13::part_2()?);
    print_result(14, &q14::part_1()?, &q14::part_2()?);
    print_result(15, &q15::part_1()?, &q15::part_2()?);
    print_result(16, &q16::part_1()?, &q16::part_2()?);
    print_result(17, &q17::part_1()?, &q17::part_2()?);
    print_result(18, &q18::part_1()?, &q18::part_2()?);
    print_result(19, &q19::part_1()?, &q19::part_2()?);
    print_result(20, &q20::part_1()?, &q20::part_2()?);
    print_result(21, &q21::part_1()?, &q21::part_2()?);
    Ok(())
}
