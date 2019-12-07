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

    // pub fn execute(&self) -> Result<&Vec<i32>, &str> {
    // }
}

impl Iterator for Intcode {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state.get(self.pointer).unwrap() {
            1 => {
                let i1 = self.state[self.pointer + 1] as usize;
                let i2 = self.state[self.pointer + 2] as usize;
                let res_i = self.state[self.pointer + 3] as usize;
                self.state[res_i] = self.state[i1] + self.state[i2];
                self.pointer += 4;
                Some(self.state.clone())
            }
            2 => {
                let i1 = self.state[self.pointer + 1] as usize;
                let i2 = self.state[self.pointer + 2] as usize;
                let res_i = self.state[self.pointer + 3] as usize;
                self.state[res_i] = self.state[i1] * self.state[i2];
                self.pointer += 4;
                Some(self.state.clone())
            }
            99 => None,
            _ => None,
        }
    }
}

pub fn part1(inp: &Vec<i32>) -> Vec<i32> {
    let computer = Intcode::new(inp.clone());
    computer.state

    // while computed[pointer] != 99 {
    //     let i1 = computed[pointer + 1];
    //     let i2 = computed[pointer + 2];
    //     let res_i = computed[pointer + 3];

    //     match computed[pointer] {
    //         1 => computed[res_i] = computed[i1] + computed[i2],
    //         2 => computed[res_i] = computed[i1] * computed[i2],
    //         _ => panic!("Invalid opcode: {}", computed[pointer]),
    //     }

    //     pointer += 4;
    // }

    // computed
}

// pub fn part2(inp: &Vec<usize>, target: usize) -> (usize, usize) {
//     let mut noun = 0;
//     let mut verb = 0;
//     'outer: for n in 0..(cmp::min(100, inp.len())) {
//         'inner: for v in 0..(cmp::min(100, inp.len())) {
//             let mut i = inp.clone();
//             i[1] = n;
//             i[2] = v;
//             if part1(&i)[0] == target {
//                 noun = n;
//                 verb = v;
//                 break 'outer;
//             }
//         }
//     }
//     (noun, verb)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode() {
        let mut computer = Intcode::new(vec![1, 0, 0, 0, 99]);
        assert_eq!(computer.next(), Some(vec![2, 0, 0, 0, 99]));
        assert_eq!(computer.next(), None);
    }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    //     assert_eq!(part1(&vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
    //     assert_eq!(part1(&vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
    //     assert_eq!(
    //         part1(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
    //         vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    //     );
    // }

    // #[test]
    // fn test_part2() {
    //     let inp = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    //     assert_eq!(part2(&inp, 2500), (2, 10));
    // }
}
