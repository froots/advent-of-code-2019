use advent_of_code_2019::d1;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./inputs/day1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let masses: Vec<usize> = contents
        .lines()
        .map(|line| match line.parse::<usize>() {
            Ok(n) => n,
            Err(_e) => 0,
        })
        .collect();
    println!("Day 1:1: {:?}", d1::total_fuel(&masses));
    Ok(())
}
