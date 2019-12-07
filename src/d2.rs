use std::cmp;

struct Intcode {
    state: Vec<i32>,
    pointer: usize,
}

impl Intcode {
    fn new(state: Vec<i32>) -> Intcode {
        Intcode {
            state: state,
            pointer: 0,
        }
    }

    fn pointer_value(&self) -> &i32 {
        self.state.get(self.pointer).unwrap()
    }

    fn get_param(&self, n: usize) -> usize {
        self.state[self.pointer + n] as usize
    }

    fn add(&mut self) -> Vec<i32> {
        let i1 = self.get_param(1);
        let i2 = self.get_param(2);
        let ri = self.get_param(3);
        self.state[ri] = self.state[i1] + self.state[i2];
        self.pointer += 4;
        self.state.clone()
    }

    fn mult(&mut self) -> Vec<i32> {
        let i1 = self.get_param(1);
        let i2 = self.get_param(2);
        let ri = self.get_param(3);
        self.state[ri] = self.state[i1] * self.state[i2];
        self.pointer += 4;
        self.state.clone()
    }
}

impl Iterator for Intcode {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pointer_value() {
            1 => Some(self.add()),
            2 => Some(self.mult()),
            99 => None,
            _ => None,
        }
    }
}

pub fn part1(inp: &Vec<i32>) -> Vec<i32> {
    let computer = Intcode::new(inp.clone());
    let states: Vec<Vec<i32>> = computer.collect();
    states.last().unwrap().clone()
}

pub fn part2(inp: &Vec<i32>, target: i32) -> (i32, i32) {
    let mut noun = 0;
    let mut verb = 0;
    'outer: for n in 0..(cmp::min(100, inp.len())) {
        'inner: for v in 0..(cmp::min(100, inp.len())) {
            let mut i = inp.clone();
            i[1] = n as i32;
            i[2] = v as i32;
            if part1(&i)[0] == target {
                noun = n as i32;
                verb = v as i32;
                break 'outer;
            }
        }
    }
    (noun, verb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode1() {
        let mut computer = Intcode::new(vec![1, 0, 0, 0, 99]);
        assert_eq!(computer.next(), Some(vec![2, 0, 0, 0, 99]));
        assert_eq!(computer.next(), None);
    }

    #[test]
    fn test_intcode2() {
        let mut computer = Intcode::new(vec![2, 3, 0, 3, 99]);
        assert_eq!(computer.next(), Some(vec![2, 3, 0, 6, 99]));
        assert_eq!(computer.next(), None);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(part1(&vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(part1(&vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            part1(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    #[test]
    fn test_part2() {
        let inp: Vec<i32> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(part2(&inp, 2500), (2, 10));
    }
}
