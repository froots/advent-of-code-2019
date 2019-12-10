use crate::intcode;

pub fn run(program_text: &str, input: i32) -> Vec<i32> {
    let program = parse(program_text);
    let mut computer = intcode::Intcode::new_with_input(program, input);
    let (_state, output) = computer.execute_with_output();
    output.clone()
}

fn parse(program_text: &str) -> Vec<i32> {
    program_text
        .trim()
        .split(',')
        .map(|i| i.parse::<i32>())
        .map(Result::unwrap)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let text = String::from(
            "1,2,4,5
",
        );
        assert_eq!(parse(&text), vec![1, 2, 4, 5]);
    }
}
