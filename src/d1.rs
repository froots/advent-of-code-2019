pub fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    input
        .lines()
        .map(|line| line.parse::<usize>().expect("Couldn't parse line"))
}

pub fn basic_fuel(masses: &[usize]) -> usize {
    masses.iter().map(&fuel_for_mass).sum()
}

pub fn total_fuel(masses: &[usize]) -> usize {
    masses.iter().map(&all_for_mass).sum()
}

fn fuel_for_mass(mass: &usize) -> usize {
    (mass / 3).max(2) - 2
}

fn all_for_mass(mass: &usize) -> usize {
    let mut total = 0;
    let mut next = *mass;
    while next > 0 {
        next = fuel_for_mass(&next);
        total += next;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_fuel() {
        let masses = [12, 14, 1969, 100756];
        assert_eq!(basic_fuel(&masses), 34241);
    }

    #[test]
    fn test_total_fuel() {
        assert_eq!(total_fuel(&[14]), 2);
        assert_eq!(total_fuel(&[1969]), 966);
        assert_eq!(total_fuel(&[100756]), 50346);
        assert_eq!(total_fuel(&[14, 1969, 100756]), 2 + 966 + 50346);
    }
}
