pub fn required_fuel(mass: i32) -> i32 {
    ((mass / 3) as i32) - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_required_fuel() {
        assert_eq!(required_fuel(12), 2);
        assert_eq!(required_fuel(14), 2);
        assert_eq!(required_fuel(1969), 654);
        assert_eq!(required_fuel(100756), 33583);
    }
}
