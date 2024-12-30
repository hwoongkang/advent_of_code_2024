use crate::Solution;

pub struct Day17;

#[derive(Debug, Clone)]
struct Computer {
    registers: [i64; 3],
    program: Vec<i64>,
    clock: usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum OperandType {
    Literal,
    Combo,
}
use Instruction::*;
use OperandType::*;

impl Instruction {
    fn from(opcode: i64) -> Self {
        match opcode {
            0 => Adv,
            1 => Bxl,
            2 => Bst,
            3 => Jnz,
            4 => Bxc,
            5 => Out,
            6 => Bdv,
            7 => Cdv,
            _ => unimplemented!(),
        }
    }
    fn operand_type(&self) -> OperandType {
        match self {
            Adv => Combo,
            Bxl => Literal,
            Bst => Combo,
            Jnz => Literal,
            Bxc => Literal,
            Out => Combo,
            Bdv => Combo,
            Cdv => Combo,
        }
    }
}

impl Computer {
    fn read_operand(&self, operand_type: OperandType, operand: i64) -> i64 {
        match operand_type {
            Literal => operand,
            Combo => {
                if operand < 4 {
                    operand
                } else if operand == 7 {
                    panic!("invalid program")
                } else {
                    let index = (operand - 4) as usize;
                    self.registers[index]
                }
            }
        }
    }
    fn from(input: String) -> Self {
        let lines = &mut input.lines();
        let a: i64 = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let b: i64 = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let c: i64 = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let registers = [a, b, c];
        let program = lines
            .nth(1)
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|w| w.parse().unwrap())
            .collect();

        Self {
            registers,
            program,
            clock: 0,
        }
    }

    fn exec(&mut self, program: (i64, i64)) -> Option<i64> {
        let (opcode, operand) = program;
        let instruction = Instruction::from(opcode);
        let operand_type = instruction.operand_type();
        let operand = self.read_operand(operand_type, operand);
        let mut new_clock = self.clock + 2;
        let mut output = None;

        match instruction {
            Adv => {
                let num = self.registers[0];
                let den = 1 << operand;
                self.registers[0] = num / den;
            }
            Bdv => {
                let num = self.registers[0];
                let dem = 1 << operand;
                self.registers[1] = num / dem;
            }
            Cdv => {
                let num = self.registers[0];
                let dem = 1 << operand;
                self.registers[2] = num / dem;
            }
            Bxl => {
                let b = &mut self.registers[1];
                *b ^= operand;
            }
            Bst => {
                self.registers[1] = operand & 0b111;
            }
            Jnz => {
                let a = self.registers[0];
                if a != 0 {
                    new_clock = operand as usize;
                }
            }
            Bxc => {
                let c = self.registers[2];
                let b = &mut self.registers[1];
                *b ^= c;
            }
            Out => output = Some(operand & 0b111),
        }
        self.clock = new_clock;
        output
    }

    fn tick(&mut self) -> (Option<i64>, bool) {
        if self.clock >= self.program.len() - 1 {
            return (None, true);
        }
        let opcode = self.program[self.clock];
        let operand = self.program[self.clock + 1];
        let p = (opcode, operand);
        let output = self.exec(p);
        (output, false)
    }

    fn output(&self, a: i64) -> Vec<i64> {
        let mut computer = self.clone();
        computer.registers[0] = a;
        let mut outputs = vec![];
        loop {
            let (output, halted) = computer.tick();

            if let Some(val) = output {
                outputs.push(val)
            }
            if halted {
                break outputs;
            }
        }
    }
}

impl Solution for Day17 {
    fn test_input() -> String {
        String::from(
            "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
        )
    }

    fn solve_part_1(input: String) -> String {
        let computer = Computer::from(input);
        computer
            .output(computer.registers[0])
            .into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn solve_part_2(input: String) -> String {
        //2,4,1,5,7,5,1,6,4,1,5,5,0,3,3,0
        // 2, 4 BST(4) => B = A % 8
        // 1, 5 BXL(5) => B ^= 101
        // 7, 5 CDV(5) => C = A >> B
        // 1, 6 BXL(6) => B ^= 110
        // 4, 1 BXC(1) => B ^= C
        // 5, 5 OUT(5) => OUT B % 8
        // 0, 3 ADV(3) => A >> 3
        // 3, 0 JNZ(0)

        // 011
        // B = 011
        // 011^101 = 110
        // A >> 110 = 0
        // 110^110 = 000
        // 0^0 = 0
        // out 0
        // 011xxx
        //

        let computer = Computer::from(input);

        fn stack_to_num(stack: &[i64]) -> i64 {
            let mut a = 0;
            for num in stack.iter() {
                a <<= 3;
                a |= num;
            }
            a
        }

        fn is_valid_stack(computer: &Computer, stack: &[i64]) -> bool {
            let a = stack_to_num(stack);
            let outputs = computer.output(a);
            let target = &computer.program[computer.program.len() - outputs.len()..];

            outputs == target
        }
        fn dfs(computer: &Computer, stack: &mut Vec<i64>) -> Option<i64> {
            for i in 0..8 {
                stack.push(i);
                if !is_valid_stack(computer, stack) {
                    stack.pop();
                    continue;
                }
                if stack.len() == computer.program.len() {
                    return Some(stack_to_num(stack));
                }
                if let Some(a) = dfs(computer, stack) {
                    return Some(a);
                } else {
                    stack.pop();
                }
            }
            None
        }

        let mut stack = vec![];
        if let Some(a) = dfs(&computer, &mut stack) {
            a.to_string()
        } else {
            panic!("Unsolvable")
        }
    }
}

#[cfg(test)]
mod day17_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day17::test_input();
        let ans = Day17::solve_part_1(input);
        assert_eq!(ans, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part_2() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
            .to_string();

        let ans = Day17::solve_part_2(input);
        assert_eq!(ans, "117440"); //11 100 101 011 000 000
    }
}
