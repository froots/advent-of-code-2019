pub fn part1(orbit_data: &str) -> u32 {
    // parse orbit data to planets
    let parsed = parse(orbit_data);
    // iterate over each planet and count orbit links to root
    1
}

fn parse(orbit_data: &str) -> Vec<(&str, &str)> {
    orbit_data
        .lines()
        .map(|line| line.split(")").collect::<Vec<&str>>())
        .map(|pair| (pair[0], pair[1]))
        .collect::<Vec<(&str, &str)>>()
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

    #[test]
    fn test_parse() {
        let data = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        assert_eq!(
            parse(&data),
            vec![
                ("COM", "B"),
                ("B", "C"),
                ("C", "D"),
                ("D", "E"),
                ("E", "F"),
                ("B", "G"),
                ("G", "H"),
                ("D", "I"),
                ("E", "J"),
                ("J", "K"),
                ("K", "L"),
            ]
        );
    }
}
