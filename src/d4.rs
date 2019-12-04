pub fn part1(lower: usize, upper: usize) -> usize {
    let mut matching = 0;
    for n in lower..=upper {
        if part1_criteria(&n) {
            matching += 1;
        }
    }
    matching
}

pub fn part2(lower: usize, upper: usize) -> usize {
    let mut matching = 0;
    for n in lower..=upper {
        if part2_criteria(&n) {
            matching += 1;
        }
    }
    matching
}

fn part1_criteria(n: &usize) -> bool {
    has_digit_size(n, 6) && has_adjacent_duplicate(n) && never_decreases(n)
}

fn part2_criteria(n: &usize) -> bool {
    has_digit_size(n, 6) && has_pair(n) && never_decreases(n)
}

fn has_digit_size(n: &usize, size: usize) -> bool {
    n.to_string().len() == size
}

fn has_adjacent_duplicate(n: &usize) -> bool {
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

fn has_pair(n: &usize) -> bool {
    let digits = n.to_string();
    let mut memo = ('_', 1);
    let mut pair = false;

    for d in digits.chars() {
        if d == memo.0 {
            memo = (d, memo.1 + 1);
        } else {
            if memo.1 == 2 {
                pair = true;
            }
            memo = (d, 1);
        }
    }

    if memo.1 == 2 {
        pair = true;
    }

    pair
}

fn never_decreases(n: &usize) -> bool {
    let mut memo: u8 = 0;
    let mut decreases = false;
    for d in n.to_string().split("") {
        let dig: u8 = match d.parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        if dig < memo {
            decreases = true;
        }
        memo = dig;
    }
    !decreases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_digit_size() {
        assert!(has_digit_size(&123456, 6));
        assert!(!has_digit_size(&123, 4));
    }

    #[test]
    fn test_has_adjacent_duplicate() {
        assert!(has_adjacent_duplicate(&123345));
        assert!(!has_adjacent_duplicate(&123456));
        assert!(has_adjacent_duplicate(&111111));
    }

    #[test]
    fn test_never_decreases() {
        assert!(never_decreases(&123344));
        assert!(!never_decreases(&234580));
    }

    #[test]
    fn test_part1_criteria() {
        assert!(part1_criteria(&111111));
        assert!(!part1_criteria(&223450));
        assert!(!part1_criteria(&123789));
    }

    #[test]
    fn test_part2_criteria() {
        assert!(part2_criteria(&112233));
        assert!(!part2_criteria(&123444));
        assert!(part2_criteria(&111122));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(123345, 123355), 6);
    }
}
