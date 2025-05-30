enum Intcode {
    Add(bool, bool, bool),
    Multiply(bool, bool, bool),
    Input,
    Output(bool),
    JumpIfTrue(bool, bool),
    JumpIfFalse(bool, bool),
    LessThan(bool, bool, bool),
    Equals(bool, bool, bool),
    Halt,
}

impl Intcode {
    fn from(mut n: i32) -> Self {
        let code = n % 100;
        match code {
            99 => Intcode::Halt,
            1 => {
                n /= 100;
                let c = n % 10 == 1;
                n /= 10;
                let b = n % 10 == 1;
                n /= 10;
                let a = n % 10 == 1;
                Intcode::Add(c, b, a)
            }
            2 => {
                n /= 100;
                let c = n % 10 == 1;
                n /= 10;
                let b = n % 10 == 1;
                n /= 10;
                let a = n % 10 == 1;
                Intcode::Multiply(c, b, a)
            }
            3 => {
                n /= 100;
                let _c = n % 10 == 1;
                Intcode::Input
            }
            4 => {
                n /= 100;
                let c = n % 10 == 1;

                Intcode::Output(c)
            }
            //
            5 => {
                n /= 100;
                let c = n % 10 == 1;
                n /= 10;
                let b = n % 10 == 1;
                Intcode::JumpIfTrue(c, b)
            }
            6 => {
                n /= 100;
                let c = n % 10 == 1;
                n /= 10;
                let b = n % 10 == 1;
                Intcode::JumpIfFalse(c, b)
            }
            7 => {
                n /= 100;
                let c = n % 10 == 1;
                n /= 10;
                let b = n % 10 == 1;
                n /= 10;
                let a = n % 10 == 1;
                Intcode::LessThan(c, b, a)
            }
            8 => {
                n /= 100;
                let c = n % 10 == 1;
                n /= 10;
                let b = n % 10 == 1;
                n /= 10;
                let a = n % 10 == 1;
                Intcode::Equals(c, b, a)
            }
            _ => panic!("Wrong Intcode"),
        }
    }
}

pub struct Computer {
    ptr: usize,
    tape: Vec<i32>,
    input_ptr: usize,
    pub input_seq: Vec<i32>,
    output: i32,
}

#[derive(PartialEq, Eq)]
pub enum Result {
    Output(i32),
    Halted(i32),
}

impl Computer {
    pub fn from(input: &str) -> Self {
        let tape = input.split(",").map(|w| w.parse().unwrap()).collect();
        let ptr = 0;
        let output = 0;
        Self {
            tape,
            ptr,
            input_seq: vec![],
            input_ptr: 0,
            output,
        }
    }

    pub fn with(program: Vec<i32>, input_seq: Vec<i32>) -> Self {
        let ptr = 0;
        let output = 0;
        let input_ptr = 0;
        Self {
            tape: program,
            input_seq,
            input_ptr,
            ptr,
            output,
        }
    }

    pub fn tick(&mut self) -> Option<Result> {
        let code = Intcode::from(self.tape[self.ptr]);
        match code {
            Intcode::Input => {
                let addr = self.tape[self.ptr + 1] as usize;
                self.tape[addr] = self.input_seq[self.input_ptr];
                self.input_ptr += 1;
                self.ptr += 2;
            }
            Intcode::Output(immediate) => {
                let tape_val = self.tape[self.ptr + 1];
                let val = if immediate {
                    tape_val
                } else {
                    self.tape[tape_val as usize]
                };
                self.output = val;
                self.ptr += 2;
                return Some(Result::Output(self.output));
            }
            Intcode::Halt => return Some(Result::Halted(self.output)),
            Intcode::Add(ia, ib, _ic) => {
                let a = self.tape[self.ptr + 1];
                let b = self.tape[self.ptr + 2];
                let c = self.tape[self.ptr + 3];

                let a = if ia { a } else { self.tape[a as usize] };
                let b = if ib { b } else { self.tape[b as usize] };
                let c = c as usize;
                self.tape[c] = a + b;
                self.ptr += 4;
            }
            Intcode::Multiply(ia, ib, _ic) => {
                let a = self.tape[self.ptr + 1];
                let b = self.tape[self.ptr + 2];
                let c = self.tape[self.ptr + 3];

                let a = if ia { a } else { self.tape[a as usize] };
                let b = if ib { b } else { self.tape[b as usize] };
                let c = c as usize;
                self.tape[c] = a * b;
                self.ptr += 4;
            }
            Intcode::JumpIfTrue(ia, ib) => {
                let a = self.tape[self.ptr + 1];
                let b = self.tape[self.ptr + 2];

                let a = if ia { a } else { self.tape[a as usize] };
                let b = if ib { b } else { self.tape[b as usize] };
                let b = b as usize;
                if a != 0 {
                    self.ptr = b;
                } else {
                    self.ptr += 3;
                }
            }
            Intcode::JumpIfFalse(ia, ib) => {
                let a = self.tape[self.ptr + 1];
                let b = self.tape[self.ptr + 2];

                let a = if ia { a } else { self.tape[a as usize] };
                let b = if ib { b } else { self.tape[b as usize] };
                let b = b as usize;
                if a == 0 {
                    self.ptr = b;
                } else {
                    self.ptr += 3;
                }
            }
            Intcode::LessThan(ia, ib, _ic) => {
                let a = self.tape[self.ptr + 1];
                let b = self.tape[self.ptr + 2];
                let c = self.tape[self.ptr + 3];

                let a = if ia { a } else { self.tape[a as usize] };
                let b = if ib { b } else { self.tape[b as usize] };
                let c = c as usize;

                let val = if a < b { 1 } else { 0 };
                self.tape[c] = val;
                self.ptr += 4;
            }
            Intcode::Equals(ia, ib, _ic) => {
                let a = self.tape[self.ptr + 1];
                let b = self.tape[self.ptr + 2];
                let c = self.tape[self.ptr + 3];

                let a = if ia { a } else { self.tape[a as usize] };
                let b = if ib { b } else { self.tape[b as usize] };
                let c = c as usize;

                let val = if a == b { 1 } else { 0 };
                self.tape[c] = val;
                self.ptr += 4;
            }
        }
        None
    }

    pub fn run_until_halt(&mut self) -> i32 {
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

    pub fn add_input(&mut self, input: i32) {
        self.input_seq.push(input);
    }
}
