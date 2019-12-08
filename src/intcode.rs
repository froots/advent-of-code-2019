pub struct Intcode {
    state: Vec<i32>,
    pointer: usize,
}

impl Intcode {
    pub fn new(state: Vec<i32>) -> Intcode {
        Intcode {
            state: state,
            pointer: 0,
        }
    }

    pub fn execute(&mut self) -> Option<Vec<i32>> {
        self.last()
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
    fn test_execute() {
        let mut computer = Intcode::new(vec![2, 3, 0, 3, 99]);
        assert_eq!(computer.execute(), Some(vec![2, 3, 0, 6, 99]));
    }
}
