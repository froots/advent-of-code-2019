pub fn part1(upper: &isize, lower: &isize) -> isize {
    1
}

fn has_digit_size(n: usize, size: usize) -> bool {
    n.to_string().len() == size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_digit_size() {
        assert!(has_digit_size(123456, 6));
        assert!(!has_digit_size(123, 4));
    }
}
