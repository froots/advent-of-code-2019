use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    Halt,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equal,
}

#[derive(PartialEq, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

pub struct Instruction {
    operation: Operation,
    p1_mode: ParameterMode,
    p2_mode: ParameterMode,
    p3_mode: ParameterMode,
}

impl Instruction {
    fn new(opcode: i32, p1_mode: i32, p2_mode: i32, p3_mode: i32) -> Instruction {
        Instruction {
            operation: match opcode {
                1 => Operation::Add,
                2 => Operation::Multiply,
                3 => Operation::Input,
                4 => Operation::Output,
                5 => Operation::JumpIfTrue,
                6 => Operation::JumpIfFalse,
                7 => Operation::LessThan,
                8 => Operation::Equal,
                99 => Operation::Halt,
                _ => panic!("Unknown operation code"),
            },
            p1_mode: Instruction::match_mode(p1_mode),
            p2_mode: Instruction::match_mode(p2_mode),
            p3_mode: Instruction::match_mode(p3_mode),
        }
    }

    fn match_mode(mode: i32) -> ParameterMode {
        match mode {
            0 => ParameterMode::Position,
            _ => ParameterMode::Immediate,
        }
    }
}

pub struct Intcode {
    state: Vec<i32>,
    input: Option<i32>,
    pub output: Vec<i32>,
    pub pointer: usize,
}

impl Intcode {
    pub fn new(state: Vec<i32>) -> Intcode {
        Intcode {
            state,
            input: None,
            output: vec![],
            pointer: 0,
        }
    }

    pub fn new_with_input(state: Vec<i32>, input: i32) -> Intcode {
        Intcode {
            state,
            input: Some(input),
            output: vec![],
            pointer: 0,
        }
    }

    pub fn execute(&mut self) -> Option<Vec<i32>> {
        self.last()
    }

    pub fn execute_with_output(&mut self) -> (Option<Vec<i32>>, &Vec<i32>) {
        (self.execute(), &self.output)
    }

    fn pointer_value(&self, offset: usize) -> Option<&i32> {
        self.state.get(self.pointer + offset)
    }

    fn get_param(&self, parameter_mode: &ParameterMode, offset: usize) -> Option<&i32> {
        let value: Option<&i32> = self.pointer_value(offset);

        match (value, parameter_mode) {
            (Some(v), ParameterMode::Position) => self.state.get(*v as usize),
            (Some(v), ParameterMode::Immediate) => Some(v),
            (None, _) => None,
        }
    }

    fn move_pointer(&mut self, n: usize) {
        self.pointer += n;
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Option<Vec<i32>> {
        let p1 = self.get_param(&instruction.p1_mode, 1);
        let p2 = self.get_param(&instruction.p2_mode, 2);

        match instruction.operation {
            Operation::Add => {
                let set_i = self.pointer_value(3).expect("Add store index").clone() as usize;
                self.state[set_i] = p1.unwrap() + p2.unwrap();
                self.move_pointer(4);
            }
            Operation::Multiply => {
                let set_i = self.pointer_value(3).expect("Multiply store index").clone() as usize;
                self.state[set_i] = p1.unwrap() * p2.unwrap();
                self.move_pointer(4);
            }
            Operation::Input => {
                let set_i = self.pointer_value(1).expect("Input store index").clone() as usize;
                self.state[set_i] = self.input.expect("Input operation requires input value");
                self.move_pointer(2);
            }
            Operation::Output => {
                let output_i = p1.expect("Need output index").clone();
                self.output.push(output_i);
                self.move_pointer(2);
            }
            Operation::JumpIfTrue => match p1.expect("Need jump flag") {
                0 => self.move_pointer(3),
                _ => self.pointer = p2.expect("Need jump destination").clone() as usize,
            },
            Operation::JumpIfFalse => match p1.expect("Need jump flag") {
                0 => self.pointer = p2.expect("Need jump destination").clone() as usize,
                _ => self.move_pointer(3),
            },
            Operation::LessThan => {
                let set_i = self
                    .pointer_value(3)
                    .expect("Less than store index")
                    .clone() as usize;
                self.state[set_i] = match p1
                    .expect("Need less than param")
                    .cmp(p2.expect("Need more than param"))
                {
                    Ordering::Less => 1,
                    _ => 0,
                };
                self.move_pointer(4);
            }
            Operation::Equal => {
                let set_i = self
                    .pointer_value(3)
                    .expect("Need equal store index")
                    .clone() as usize;
                self.state[set_i] = match p1
                    .expect("Need equal param 1")
                    .cmp(p2.expect("Need equal param 2"))
                {
                    Ordering::Equal => 1,
                    _ => 0,
                };
                self.move_pointer(4);
            }
            _ => {}
        }

        match instruction.operation {
            Operation::Halt => None,
            _ => Some(self.state.clone()),
        }
    }
}

impl Iterator for Intcode {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction =
            parse_instruction(self.pointer_value(0).expect("No instruction to parse"));
        self.execute_instruction(instruction)
    }
}

fn parse_instruction(code: &i32) -> Instruction {
    let p3_mode = code / 10_000;
    let mut remain = code % 10_000;
    let p2_mode = remain / 1_000;
    remain = remain % 1_000;
    let p1_mode = remain / 100;
    remain = remain % 100;

    Instruction::new(remain, p1_mode, p2_mode, p3_mode)
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
    fn test_intcode3() {
        let mut computer = Intcode::new_with_input(vec![3, 0, 4, 0, 99], 1);
        assert_eq!(computer.next(), Some(vec![1, 0, 4, 0, 99]));
    }

    #[test]
    fn test_intcode4() {
        let mut computer = Intcode::new(vec![4, 2, 99]);
        assert_eq!(computer.output, vec![]);
        assert_eq!(computer.next(), Some(vec![4, 2, 99]));
        assert_eq!(computer.next(), None);
        assert_eq!(computer.output, vec![99]);
    }

    #[test]
    fn test_intcode5_non_zero() {
        let mut computer = Intcode::new(vec![5, 1, 3, 7, 2, 3, 2, 99]);
        assert_eq!(computer.pointer, 0);
        assert_eq!(computer.next(), Some(vec![5, 1, 3, 7, 2, 3, 2, 99]));
        assert_eq!(computer.pointer, 7);
        assert_eq!(computer.next(), None);
    }

    #[test]
    fn test_intcode5_zero() {
        let mut computer = Intcode::new(vec![5, 8, 3, 7, 2, 3, 2, 99, 0]);
        assert_eq!(computer.pointer, 0);
        assert_eq!(computer.next(), Some(vec![5, 8, 3, 7, 2, 3, 2, 99, 0]));
        assert_eq!(computer.pointer, 3);
    }

    #[test]
    fn test_intcode6_non_zero() {
        let mut computer = Intcode::new(vec![6, 8, 3, 7, 2, 3, 2, 99, 1]);
        assert_eq!(computer.pointer, 0);
        assert_eq!(computer.next(), Some(vec![6, 8, 3, 7, 2, 3, 2, 99, 1]));
        assert_eq!(computer.pointer, 3);
    }

    #[test]
    fn test_intcode7_less_than() {
        let mut computer = Intcode::new(vec![7, 7, 8, 7, 4, 7, 99, 3, 8]);
        assert_eq!(computer.next(), Some(vec![7, 7, 8, 7, 4, 7, 99, 1, 8]));
    }

    #[test]
    fn test_intcode7_greater_than() {
        let mut computer = Intcode::new(vec![7, 7, 8, 7, 4, 7, 99, 9, 8]);
        assert_eq!(computer.next(), Some(vec![7, 7, 8, 7, 4, 7, 99, 0, 8]));
    }

    #[test]
    fn test_intcode8_equal() {
        let mut computer = Intcode::new(vec![8, 5, 6, 0, 99, 4, 4]);
        assert_eq!(computer.next(), Some(vec![1, 5, 6, 0, 99, 4, 4]));
    }

    #[test]
    fn test_intcode8_not_equal() {
        let mut computer = Intcode::new(vec![8, 5, 6, 0, 99, 5, 4]);
        assert_eq!(computer.next(), Some(vec![0, 5, 6, 0, 99, 5, 4]));
    }

    #[test]
    fn test_execute() {
        let mut computer = Intcode::new(vec![2, 3, 0, 3, 99]);
        assert_eq!(computer.execute(), Some(vec![2, 3, 0, 6, 99]));
    }

    #[test]
    fn test_execute_with_output() {
        let mut computer = Intcode::new_with_input(vec![3, 0, 4, 0, 99], 12);
        let (_, output) = computer.execute_with_output();
        assert_eq!(output, &vec![12]);
    }

    #[test]
    fn test_parse_instruction1() {
        let instruction = parse_instruction(&1002);
        assert_eq!(instruction.operation, Operation::Multiply);
        assert_eq!(instruction.p1_mode, ParameterMode::Position);
        assert_eq!(instruction.p2_mode, ParameterMode::Immediate);
        assert_eq!(instruction.p3_mode, ParameterMode::Position);
    }

    #[test]
    fn test_parse_instruction2() {
        let instruction = parse_instruction(&10101);
        assert_eq!(instruction.operation, Operation::Add);
        assert_eq!(instruction.p1_mode, ParameterMode::Immediate);
        assert_eq!(instruction.p2_mode, ParameterMode::Position);
        assert_eq!(instruction.p3_mode, ParameterMode::Immediate);
    }

    #[test]
    fn test_parse_instruction3() {
        let instruction = parse_instruction(&4);
        assert_eq!(instruction.operation, Operation::Output);
        assert_eq!(instruction.p1_mode, ParameterMode::Position);
        assert_eq!(instruction.p2_mode, ParameterMode::Position);
        assert_eq!(instruction.p3_mode, ParameterMode::Position);
    }

    #[test]
    fn test_parameter_modes1() {
        let mut computer = Intcode::new(vec![102, 5, 3, 2, 99]);
        assert_eq!(computer.next(), Some(vec![102, 5, 10, 2, 99]));
    }

    // Provided input-output tests from day 5
    #[test]
    fn test_input_output_examples() {
        let examples = vec![
            (vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8, vec![1]),
            (vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 7, vec![0]),
            (vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 7, vec![1]),
            (vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 8, vec![0]),
            (vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 8, vec![1]),
            (vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 7, vec![0]),
            (vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 7, vec![1]),
            (vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 8, vec![0]),
            (
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                0,
                vec![0],
            ),
            (
                vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                5,
                vec![1],
            ),
            (
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                0,
                vec![0],
            ),
            (
                vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
                9,
                vec![1],
            ),
            (
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
                ],
                7,
                vec![999],
            ),
            (
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
                ],
                8,
                vec![1000],
            ),
            (
                vec![
                    3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0,
                    36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46,
                    1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
                ],
                9,
                vec![1001],
            ),
        ];

        for (program, input, expected_output) in examples {
            let mut computer = Intcode::new_with_input(program, input);
            let (_, output) = computer.execute_with_output();
            assert_eq!(output, &expected_output);
        }
    }
}
