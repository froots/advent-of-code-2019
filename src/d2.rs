pub fn part1(inp: &mut Vec<i64>) -> i64 {
    inp[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut inp1 = vec![1, 0, 0, 0, 99];
        assert_eq!(part1(&mut inp1), 1);
    }
}
