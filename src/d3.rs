use std::collections::HashSet;

pub fn part1(input: &str) -> isize {
    let wires: Vec<HashSet<(isize, isize)>> = input
        .lines()
        .map(|wire_input| parse_wire_input(&wire_input))
        .collect();
    let wire1 = wires.get(0).unwrap();
    let wire2 = wires.get(1).unwrap();
    let mut distances: Vec<isize> = wire1
        .intersection(&wire2)
        .cloned()
        .map(|(x, y)| x.abs() + y.abs())
        .collect();
    distances.sort();
    *distances.first().unwrap()
}

fn parse_wire_input(input: &str) -> HashSet<(isize, isize)> {
    let mut wire = HashSet::new();
    let mut px = 0;
    let mut py = 0;

    for instruction in input.split(',') {
        let (dx, dy) = match &instruction[..1] {
            "U" => (0, 1),
            "D" => (0, -1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => panic!("Unknown direction"),
        };
        let distance: isize = instruction[1..].parse::<isize>().unwrap();

        for _ in 0..distance {
            px += dx;
            py += dy;
            wire.insert((px, py));
        }
    }
    wire
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wire_input() {
        let wire: HashSet<(isize, isize)> = parse_wire_input("R3,U2,L2,D2");
        assert_eq!(wire.len(), 8);
        assert!(wire.contains(&(1, 0)));
        assert!(wire.contains(&(2, 0)));
        assert!(wire.contains(&(3, 0)));
        assert!(wire.contains(&(3, 1)));
        assert!(wire.contains(&(3, 2)));
        assert!(wire.contains(&(2, 2)));
        assert!(wire.contains(&(1, 2)));
        assert!(wire.contains(&(1, 1)));
    }
}
