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

    #[test]
    fn test_part1() {
        let inp1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let inp2 =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(part1(&inp1), 159);
        assert_eq!(part1(&inp2), 135);
    }
}
