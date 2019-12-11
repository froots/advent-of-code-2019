pub fn part1(orbit_data: &str) -> u32 {
    // parse orbit data to planets
    // iterate over each planet and count orbit links to root
    1
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_example() {
        let data = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        assert_eq!(part1(&data), 42);
    }
}
