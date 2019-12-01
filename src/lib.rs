pub fn required_fuel(mass: &usize) -> usize {
    ((mass / 3) as usize) - 2
}

pub fn total_fuel(masses: &[usize]) -> usize {
    masses.iter().map(|mass| required_fuel(&mass)).sum()
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
