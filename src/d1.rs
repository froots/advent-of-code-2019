pub fn total_fuel(masses: &[usize]) -> usize {
    masses.iter().map(|mass| mass / 3 - 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_total_fuel() {
        let masses = [12, 14, 1969, 100756];
        assert_eq!(total_fuel(&masses), 34241);
    }
}
