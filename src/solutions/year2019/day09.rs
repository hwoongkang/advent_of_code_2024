use crate::Solution;

use super::computer::Computer;

pub struct Day09 {}

impl Solution for Day09 {
    fn test_input() -> String {
        String::new()
    }
    fn solve_part_1(input: String) -> String {
        let mut computer = Computer::from(&input);
        computer.add_input(1);
        let output = computer.run_until_halt();
        output.to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut computer = Computer::from(&input);
        computer.add_input(2);
        let output = computer.run_until_halt();
        output.to_string()
    }
}

#[cfg(test)]
mod day09_tests {}
