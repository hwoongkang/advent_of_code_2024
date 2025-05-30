#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn from(n: i64) -> Self {
        match n {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => panic!("Wrong Parameter Mode"),
        }
    }
}

#[derive(Debug)]
enum Intcode {
    Add(ParameterMode, ParameterMode, ParameterMode),
    Multiply(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
    Halt,
    Offset(ParameterMode),
}

impl Intcode {
    fn from(mut n: i64) -> Self {
        let code = n % 100;
        match code {
            99 => Intcode::Halt,
            1 => {
                n /= 100;
                let c = ParameterMode::from(n % 10);
                n /= 10;
                let b = ParameterMode::from(n % 10);
                n /= 10;
                let a = ParameterMode::from(n % 10);
                Intcode::Add(c, b, a)
            }
            2 => {
                n /= 100;
                let c = ParameterMode::from(n % 10);
                n /= 10;
                let b = ParameterMode::from(n % 10);
                n /= 10;
                let a = ParameterMode::from(n % 10);

                Intcode::Multiply(c, b, a)
            }
            3 => {
                n /= 100;
                let c = ParameterMode::from(n % 10);
                Intcode::Input(c)
            }
            4 => {
                n /= 100;
                let c = ParameterMode::from(n % 10);

                Intcode::Output(c)
            }
            //
            5 => {
                n /= 100;
                let c = ParameterMode::from(n % 10);
                n /= 10;
                let b = ParameterMode::from(n % 10);
                Intcode::JumpIfTrue(c, b)
            }
            6 => {
                n /= 100;
                let c = ParameterMode::from(n % 10);
                n /= 10;
                let b = ParameterMode::from(n % 10);
                Intcode::JumpIfFalse(c, b)
            }
            7 => {
                n /= 100;
                let c = ParameterMode::from(n % 10);
                n /= 10;
                let b = ParameterMode::from(n % 10);
                n /= 10;
                let a = ParameterMode::from(n % 10);
                Intcode::LessThan(c, b, a)
            }
            8 => {
                n /= 100;
                let c = ParameterMode::from(n % 10);
                n /= 10;
                let b = ParameterMode::from(n % 10);
                n /= 10;
                let a = ParameterMode::from(n % 10);
                Intcode::Equals(c, b, a)
            }
            9 => {
                n /= 100;
                let c = ParameterMode::from(n % 10);
                Intcode::Offset(c)
            }
            _ => panic!("Wrong Intcode"),
        }
    }
}

pub struct Computer {
    ptr: usize,
    tape: Vec<i64>,
    input_ptr: usize,
    pub input_seq: Vec<i64>,
    relative_base: i64,
    output: i64,
}

#[derive(PartialEq, Eq)]
pub enum Result {
    Output(i64),
    Halted(i64),
}

impl Computer {
    fn try_read(&self, ptr: usize) -> i64 {
        if ptr >= self.tape.len() {
            0
        } else {
            self.tape[ptr]
        }
    }
    fn read_from(&self, ptr: usize, mode: ParameterMode) -> i64 {
        let val = self.try_read(ptr);
        match mode {
            ParameterMode::Immediate => val,
            ParameterMode::Position => self.try_read(val as usize),
            ParameterMode::Relative => self.try_read((self.relative_base + val) as usize),
        }
    }

    fn try_write(&mut self, ptr: usize, value: i64) {
        while self.tape.len() <= ptr {
            self.tape.push(0);
        }
        self.tape[ptr] = value;
    }

    fn write_to(&mut self, ptr: usize, mode: ParameterMode, value: i64) {
        let val = self.try_read(ptr);
        match mode {
            ParameterMode::Immediate => panic!("Write Parameter Should Not Be Immediate"),
            ParameterMode::Position => {
                self.try_write(val as usize, value);
            }
            ParameterMode::Relative => {
                self.try_write((self.relative_base + val) as usize, value);
            }
        }
    }
    pub fn from(input: &str) -> Self {
        let tape = input.split(",").map(|w| w.parse().unwrap()).collect();
        let ptr = 0;
        let output = 0;
        Self {
            tape,
            ptr,
            input_seq: vec![],
            input_ptr: 0,
            relative_base: 0,
            output,
        }
    }

    pub fn with(program: Vec<i64>, input_seq: Vec<i64>) -> Self {
        let ptr = 0;
        let output = 0;
        let input_ptr = 0;
        Self {
            tape: program,
            input_seq,
            input_ptr,
            ptr,
            relative_base: 0,
            output,
        }
    }

    pub fn tick(&mut self) -> Option<Result> {
        let code = Intcode::from(self.tape[self.ptr]);
        // println!("tick! ptr: {} code: {:?}", self.ptr, code);
        match code {
            Intcode::Input(mode) => {
                let input = self.input_seq[self.input_ptr];
                self.input_ptr += 1;
                self.write_to(self.ptr + 1, mode, input);
                self.ptr += 2;
            }
            Intcode::Output(mode) => {
                let val = self.read_from(self.ptr + 1, mode);
                self.output = val;
                self.ptr += 2;
                return Some(Result::Output(self.output));
            }
            Intcode::Halt => return Some(Result::Halted(self.output)),
            Intcode::Add(a, b, c) => {
                let a = self.read_from(self.ptr + 1, a);
                let b = self.read_from(self.ptr + 2, b);
                let value = a + b;
                self.write_to(self.ptr + 3, c, value);
                self.ptr += 4;
            }
            Intcode::Multiply(a, b, c) => {
                let a = self.read_from(self.ptr + 1, a);
                let b = self.read_from(self.ptr + 2, b);
                let value = a * b;
                self.write_to(self.ptr + 3, c, value);
                self.ptr += 4;
            }
            Intcode::JumpIfTrue(a, b) => {
                let a = self.read_from(self.ptr + 1, a);
                let b = self.read_from(self.ptr + 2, b);

                let b = b as usize;
                if a != 0 {
                    self.ptr = b;
                } else {
                    self.ptr += 3;
                }
            }
            Intcode::JumpIfFalse(a, b) => {
                let a = self.read_from(self.ptr + 1, a);
                let b = self.read_from(self.ptr + 2, b);

                let b = b as usize;
                if a == 0 {
                    self.ptr = b;
                } else {
                    self.ptr += 3;
                }
            }
            Intcode::LessThan(a, b, c) => {
                let a = self.read_from(self.ptr + 1, a);
                let b = self.read_from(self.ptr + 2, b);
                let value = if a < b { 1 } else { 0 };
                self.write_to(self.ptr + 3, c, value);
                self.ptr += 4;
            }
            Intcode::Equals(a, b, c) => {
                let a = self.read_from(self.ptr + 1, a);
                let b = self.read_from(self.ptr + 2, b);
                let value = if a == b { 1 } else { 0 };
                self.write_to(self.ptr + 3, c, value);
                self.ptr += 4;
            }
            Intcode::Offset(a) => {
                let a = self.read_from(self.ptr + 1, a);
                self.relative_base += a;
                self.ptr += 2;
            }
        }
        None
    }

    pub fn run_until_halt(&mut self) -> i64 {
        loop {
            if let Some(Result::Halted(output)) = self.tick() {
                break output;
            }
        }
    }

    pub fn run(&mut self) -> Result {
        loop {
            if let Some(result) = self.tick() {
                break result;
            }
        }
    }

    pub fn add_input(&mut self, input: i64) {
        self.input_seq.push(input);
    }
}

#[cfg(test)]
mod computer_tests {
    use super::*;

    #[test]
    fn test_day_5() {
        let program = String::from("3,9,8,9,10,9,4,9,99,-1,8");
        let program = program.split(",").map(|w| w.parse().unwrap()).collect();
        let input_seq = vec![8];
        let mut computer = Computer::with(program, input_seq);
        let output = computer.run_until_halt();
        assert_eq!(output, 1)
    }

    #[test]
    fn test_day_9_1() {
        let input = String::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let mut computer = Computer::from(&input);
        let mut ans = vec![];
        loop {
            match computer.run() {
                Result::Output(output) => ans.push(output.to_string()),
                Result::Halted(_) => break,
            }
        }
        assert_eq!(ans.join(","), input)
    }

    #[test]
    fn test_day_9_2() {
        let input = String::from("104,1125899906842624,99");
        let mut computer = Computer::from(&input);
        let mut ans = vec![];
        loop {
            match computer.run() {
                Result::Output(output) => ans.push(output.to_string()),
                Result::Halted(_) => break,
            }
        }
        assert_eq!(ans.join("\n"), "1125899906842624")
    }
}
