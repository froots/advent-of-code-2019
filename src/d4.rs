pub fn part1(upper: &isize, lower: &isize) -> isize {
    1
}

fn has_digit_size(n: usize, size: usize) -> bool {
    n.to_string().len() == size
}

fn has_adjacent_duplicate(n: usize) -> bool {
    let digits = n.to_string();
    let mut memo = '_';
    let mut dupe = false;

    for d in digits.chars() {
        if d == memo {
            dupe = true;
        }
        memo = d;
    }

    dupe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_digit_size() {
        assert!(has_digit_size(123456, 6));
        assert!(!has_digit_size(123, 4));
    }

    #[test]
    fn test_has_adjacent_duplicate() {
        assert!(has_adjacent_duplicate(123345));
        assert!(!has_adjacent_duplicate(123456));
        assert!(has_adjacent_duplicate(111111));
    }
}
