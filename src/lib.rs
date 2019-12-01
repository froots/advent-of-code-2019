pub fn required_fuel(mass: &usize) -> usize {
    ((mass / 3) as usize) - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_required_fuel() {
        let data = [(12, 2), (14, 2), (1969, 654), (100756, 33583)];

        for (mass, expected_fuel) in data.iter() {
            assert_eq!(required_fuel(mass), *expected_fuel);
        }
    }
}
