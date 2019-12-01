pub fn basic_fuel(masses: &[usize]) -> usize {
    masses.iter().map(|mass| mass / 3 - 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_basic_fuel() {
        let masses = [12, 14, 1969, 100756];
        assert_eq!(basic_fuel(&masses), 34241);
    }
}
