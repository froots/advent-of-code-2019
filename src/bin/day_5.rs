use advent_of_code_2019::d5;
use advent_of_code_2019::input;

fn main() -> std::io::Result<()> {
    let program = input::read("./inputs/day5.txt").expect("Failed to read file");

    println!("Day 5:1: {}", d5::part1(&program, 1));
    Ok(())
}
