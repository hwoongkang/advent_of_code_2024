use super::computer::Computer;
use crate::Solution;

pub struct Day05 {}

impl Solution for Day05 {
    fn test_input() -> String {
        String::new()
    }
    fn solve_part_1(input: String) -> String {
        let tape: Vec<i32> = input.split(",").map(|w| w.parse().unwrap()).collect();
        let input_seq = vec![1];
        let mut computer = Computer::with(tape, input_seq);

        computer.run_until_halt().to_string()
    }
    fn solve_part_2(input: String) -> String {
        let tape: Vec<i32> = input.split(",").map(|w| w.parse().unwrap()).collect();
        let input_seq = vec![5];
        let mut computer = Computer::with(tape, input_seq);

        computer.run_until_halt().to_string()
    }
}

#[cfg(test)]
mod day05_tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let program = String::from("3,9,8,9,10,9,4,9,99,-1,8");
        let program = program.split(",").map(|w| w.parse().unwrap()).collect();
        let input_seq = vec![8];
        let mut computer = Computer::with(program, input_seq);
        let output = computer.run_until_halt();
        assert_eq!(output, 1)
    }
}
