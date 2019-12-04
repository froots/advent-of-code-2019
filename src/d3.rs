use std::collections::HashMap;
use std::collections::HashSet;

type Wire = HashMap<(isize, isize), isize>;

pub fn part1(input: &str) -> isize {
    let (wire1, wire2) = parse_wires(input);
    let intersections = get_intersections(&wire1, &wire2);
    let mut distances: Vec<isize> = intersections
        .into_iter()
        .map(|(x, y)| x.abs() + y.abs())
        .collect();
    distances.sort();
    *distances.first().unwrap()
}

pub fn part2(input: &str) -> isize {
    let (wire1, wire2) = parse_wires(input);
    1
}

fn get_intersections(wire1: &Wire, wire2: &Wire) -> Vec<(isize, isize)> {
    let wire_set1: HashSet<(isize, isize)> = wire1.keys().cloned().collect();
    let wire_set2: HashSet<(isize, isize)> = wire2.keys().cloned().collect();
    wire_set1.intersection(&wire_set2).cloned().collect()
}

fn parse_wires(input: &str) -> (Wire, Wire) {
    let wires: Vec<Wire> = input
        .lines()
        .map(|wire_input| parse_wire_input(&wire_input))
        .collect();
    (wires.get(0).unwrap().clone(), wires.get(1).unwrap().clone())
}

fn parse_wire_input(input: &str) -> Wire {
    let mut wire = Wire::new();
    let mut px = 0;
    let mut py = 0;
    let mut steps = 0;

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
            steps += 1;
            if !wire.contains_key(&(px, py)) {
                wire.insert((px, py), steps);
            }
        }
    }
    wire
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let inp1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let inp2 =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(part1(&inp1), 159);
        assert_eq!(part1(&inp2), 135);
    }

    // #[test]
    // fn test_part2() {
    //     let inp1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    //     let inp2 =
    //         "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    //     assert_eq!(part2(&inp1), 610);
    //     assert_eq!(part2(&inp2), 410);
    // }
}
