use advent_of_code_2019::d3;
use advent_of_code_2019::input;

fn main() -> std::io::Result<()> {
    let inp = input::read("./inputs/day3.txt").expect("Failed to read file");
    println!("Day 3:1: {}", d3::part1(&inp));
    Ok(())
}
