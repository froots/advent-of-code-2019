use advent_of_code_2019::d6;
use advent_of_code_2019::input;

fn main() -> std::io::Result<()> {
    let data = input::read("./inputs/day6.txt").expect("Failed to read file");

    println!("Day 6:1: {:?}", d6::part1(&data));
    Ok(())
}
