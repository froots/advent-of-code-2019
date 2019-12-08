pub struct Intcode {
    state: Vec<i32>,
    input: Option<i32>,
    pub output: Option<i32>,
    pointer: usize,
}

impl Intcode {
    pub fn new(state: Vec<i32>) -> Intcode {
        Intcode {
            state,
            input: None,
            output: None,
            pointer: 0,
        }
    }

    pub fn new_with_input(state: Vec<i32>, input: i32) -> Intcode {
        Intcode {
            state,
            input: Some(input),
            output: None,
            pointer: 0,
        }
    }

    pub fn execute(&mut self) -> Option<Vec<i32>> {
        self.last()
    }

    pub fn execute_with_output(&mut self) -> (Option<Vec<i32>>, Option<i32>) {
        (self.execute(), self.output)
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

    fn input(&mut self) -> Vec<i32> {
        if self.input == None {
            panic!("Input opcode requires a defined input prop");
        }
        let i = self.get_param(1);
        self.state[i] = self.input.unwrap();
        self.pointer += 2;
        self.state.clone()
    }

    fn output(&mut self) -> Vec<i32> {
        if self.output != None {
            panic!("Program already created output");
        }
        let i = self.get_param(1);
        self.output = self.state.get(i).cloned();
        self.pointer += 2;
        self.state.clone()
    }
}

impl Iterator for Intcode {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pointer_value() {
            1 => Some(self.add()),
            2 => Some(self.mult()),
            3 => Some(self.input()),
            4 => Some(self.output()),
            99 => None,
            _ => None,
        }
    }
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
        assert_eq!(computer.output, None);
        assert_eq!(computer.next(), Some(vec![4, 2, 99]));
        assert_eq!(computer.next(), None);
        assert_eq!(computer.output, Some(99));
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
        assert_eq!(output, Some(12));
    }
}
