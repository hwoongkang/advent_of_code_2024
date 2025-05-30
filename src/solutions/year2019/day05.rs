use crate::Solution;

pub struct Day05 {}

#[derive(Debug)]
enum Opcode {
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

impl Opcode {
    fn from(mut n: i32) -> Self {
        let code = n % 100;
        match code {
            99 => Opcode::Halt,
            1 => {
                n /= 100;
                let c = n % 10 == 1;
                n /= 10;
                let b = n % 10 == 1;
                n /= 10;
                let a = n % 10 == 1;
                Opcode::Add(c, b, a)
            }
            2 => {
                n /= 100;
                let c = n % 10 == 1;
                n /= 10;
                let b = n % 10 == 1;
                n /= 10;
                let a = n % 10 == 1;
                Opcode::Multiply(c, b, a)
            }
            3 => {
                n /= 100;
                let _c = n % 10 == 1;
                Opcode::Input
            }
            4 => {
                n /= 100;
                let c = n % 10 == 1;

                Opcode::Output(c)
            }
            //
            5 => {
                n /= 100;
                let c = n % 10 == 1;
                n /= 10;
                let b = n % 10 == 1;
                Opcode::JumpIfTrue(c, b)
            }
            6 => {
                n /= 100;
                let c = n % 10 == 1;
                n /= 10;
                let b = n % 10 == 1;
                Opcode::JumpIfFalse(c, b)
            }
            7 => {
                n /= 100;
                let c = n % 10 == 1;
                n /= 10;
                let b = n % 10 == 1;
                n /= 10;
                let a = n % 10 == 1;
                Opcode::LessThan(c, b, a)
            }
            8 => {
                n /= 100;
                let c = n % 10 == 1;
                n /= 10;
                let b = n % 10 == 1;
                n /= 10;
                let a = n % 10 == 1;
                Opcode::Equals(c, b, a)
            }
            _ => panic!(),
        }
    }

    fn exec(&self, pointer: &mut usize, tape: &mut [i32]) -> Option<()> {
        match self {
            Self::Input => {
                println!("Input: {:?}", &tape[*pointer..*pointer + 2]);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let addr = tape[*pointer + 1] as usize;

                tape[addr] = input.trim().parse().unwrap();
                *pointer += 2;
                None
            }
            Self::Output(immediate) => {
                println!("Output: {:?}", &tape[*pointer..*pointer + 2]);
                let tape_val = tape[*pointer + 1];
                let val = if *immediate {
                    tape_val
                } else {
                    tape[tape_val as usize]
                };
                if val != 0 {
                    let mut _input = String::new();
                    println!("output is not 0: {} {}", *pointer + 1, val);
                    std::io::stdin().read_line(&mut _input).unwrap();
                }
                println!("Output: {} {}", *pointer + 1, val);
                *pointer += 2;
                None
            }
            Self::Add(ia, ib, _ic) => {
                println!("add: {:?}", &tape[(*pointer)..(*pointer + 4)]);
                let a = tape[*pointer + 1];
                let b = tape[*pointer + 2];
                let c = tape[*pointer + 3];

                let a = if *ia { a } else { tape[a as usize] };
                let b = if *ib { b } else { tape[b as usize] };

                let c = c as usize;

                println!("Adding {} + {} to {}", a, b, c);
                tape[c] = a + b;
                *pointer += 4;
                None
            }
            Self::Multiply(ia, ib, _ic) => {
                println!("mul: {:?}", &tape[(*pointer)..(*pointer + 4)]);
                let a = tape[*pointer + 1];
                let b = tape[*pointer + 2];
                let c = tape[*pointer + 3];

                let a = if *ia { a } else { tape[a as usize] };
                let b = if *ib { b } else { tape[b as usize] };

                let c = c as usize;
                println!("Multiplying {} * {} to {}", a, b, c);

                tape[c] = a * b;
                *pointer += 4;
                None
            }
            Self::JumpIfTrue(ia, ib) => {
                println!("JumpIfTrue: {:?}", &tape[*pointer..*pointer + 3]);
                let a = tape[*pointer + 1];
                let b = tape[*pointer + 2];
                let a = if *ia { a } else { tape[a as usize] };
                let b = if *ib { b } else { tape[b as usize] };
                let b = b as usize;
                if a != 0 {
                    *pointer = b
                } else {
                    *pointer += 3;
                }
                None
            }
            Self::JumpIfFalse(ia, ib) => {
                println!("JumpIfFalse: {:?}", &tape[*pointer..*pointer + 3]);
                let a = tape[*pointer + 1];
                let b = tape[*pointer + 2];
                let a = if *ia { a } else { tape[a as usize] };
                let b = if *ib { b } else { tape[b as usize] };
                let b = b as usize;
                if a == 0 {
                    *pointer = b
                } else {
                    *pointer += 3;
                }
                None
            }
            Self::LessThan(ia, ib, _ic) => {
                println!("LessThan: {:?}", &tape[(*pointer)..(*pointer + 4)]);
                let a = tape[*pointer + 1];
                let b = tape[*pointer + 2];
                let c = tape[*pointer + 3];

                let a = if *ia { a } else { tape[a as usize] };
                let b = if *ib { b } else { tape[b as usize] };

                let c = c as usize;
                let new_val = if a < b { 1 } else { 0 };

                tape[c] = new_val;
                *pointer += 4;
                None
            }
            Self::Equals(ia, ib, _ic) => {
                println!("Equals: {:?}", &tape[(*pointer)..(*pointer + 4)]);
                let a = tape[*pointer + 1];
                let b = tape[*pointer + 2];
                let c = tape[*pointer + 3];

                let a = if *ia { a } else { tape[a as usize] };
                let b = if *ib { b } else { tape[b as usize] };

                let c = c as usize;
                let new_val = if a == b { 1 } else { 0 };

                tape[c] = new_val;
                *pointer += 4;
                None
            }
            Self::Halt => Some(()),
        }
    }
}

impl Solution for Day05 {
    fn test_input() -> String {
        String::new()
    }
    fn solve_part_1(input: String) -> String {
        let mut tape: Vec<i32> = input.split(",").map(|w| w.parse().unwrap()).collect();
        let mut pointer = 0;
        loop {
            let op_code = Opcode::from(tape[pointer]);
            println!();
            println!("pointer: {}, code: {:?}", pointer, &op_code);
            if let Some(_) = op_code.exec(&mut pointer, &mut tape) {
                break String::new();
            }
        }
    }
    fn solve_part_2(input: String) -> String {
        let mut tape: Vec<i32> = input.split(",").map(|w| w.parse().unwrap()).collect();
        let mut pointer = 0;
        loop {
            let op_code = Opcode::from(tape[pointer]);
            println!();
            println!("pointer: {}, code: {:?}", pointer, &op_code);
            if let Some(_) = op_code.exec(&mut pointer, &mut tape) {
                break String::new();
            }
        }
    }
}

#[cfg(test)]
mod day05_tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let program = String::from("3,9,8,9,10,9,4,9,99,-1,8");
        println!("type 8");
        let ans = Day05::solve_part_2(program);
        assert_eq!(ans, "0")
    }
}
