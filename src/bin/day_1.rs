use advent_of_code_2019::d1;
use advent_of_code_2019::input;

fn main() -> std::io::Result<()> {
    let inp = input::read("./inputs/day1.txt").expect("Couldn't read file");
    let masses: Vec<usize> = d1::parse(&inp).collect();
    println!("Day 1:1: {}", d1::basic_fuel(&masses));
    println!("Day 1:2: {}", d1::total_fuel(&masses));
    Ok(())
}
