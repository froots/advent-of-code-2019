use advent_of_code_2019::d1::total_fuel;

fn main() {
    let masses = [45, 12, 923];
    println!("Required fuel: {}", total_fuel(&masses));
}
