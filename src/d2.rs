pub fn part1(inp: &Vec<usize>) -> Vec<usize> {
    let mut computed = inp.clone();
    let mut pointer = 0;

    while computed[pointer] != 99 {
        let i1 = computed[pointer + 1];
        let i2 = computed[pointer + 2];
        let res_i = computed[pointer + 3];

        match computed[pointer] {
            1 => computed[res_i] = computed[i1] + computed[i2],
            2 => computed[res_i] = computed[i1] * computed[i2],
            _ => panic!("Invalid opcode: {}", computed[pointer]),
        }

        pointer += 4;
    }

    computed
}

pub fn part2(inp: &Vec<usize>, target: usize) -> (usize, usize) {
    let mut noun = 0;
    let mut verb = 0;
    'outer: for n in 0..100 {
        'inner: for v in 0..100 {
            let mut i = inp.clone();
            i[1] = n;
            i[2] = v;
            if part1(&i)[0] == target {
                noun = n;
                verb = v;
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
    fn test_part1() {
        assert_eq!(part1(&vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(part1(&vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(part1(&vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            part1(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
