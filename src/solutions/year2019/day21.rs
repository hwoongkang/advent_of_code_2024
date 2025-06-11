use crate::Solution;

use super::computer::Computer;

pub struct Day21 {}

fn to_ascii(c: char) -> i64 {
    (c as u32) as i64
}

fn from_ascii(n: i64) -> char {
    char::from_u32(n as u32).unwrap()
}

impl Solution for Day21 {
    fn test_input() -> String {
        String::new()
    }
    fn solve_part_1(input: String) -> String {
        let mut computer = Computer::from(&input);
        // A B C D
        // if D
        // AND (!A || !B || !C)
        // NOT A T
        // NOT B J
        // OR T J J = !B || !A
        // NOT C T
        // OR T J
        // AND D J
        let commands = [
            "NOT A T", "NOT B J", "OR T J", "NOT C T", "OR T J", "AND D J",
        ];
        for ch in commands.join("\n").chars() {
            computer.add_input(to_ascii(ch));
        }
        for ch in "\nWALK\n".chars() {
            computer.add_input(to_ascii(ch));
        }
        let mut outputs = vec![];
        let halted = loop {
            match computer.run() {
                super::computer::Result::NeedsInput => panic!("ENOUGH INPUT ALREADY"),
                super::computer::Result::Output(n) => outputs.push(n),
                super::computer::Result::Halted(h) => break h,
            }
        };
        println!(
            "{}",
            outputs
                .iter()
                .map(|&n| n)
                .take_while(|&n| match char::from_u32(n as u32) {
                    Some(_) => true,
                    _ => false,
                })
                .map(from_ascii)
                .collect::<String>()
        );
        println!("{}", halted);
        if halted < (u32::MAX as i64) {
            outputs
                .into_iter()
                .take_while(|n| match char::from_u32(*n as u32) {
                    Some(_) => true,
                    _ => false,
                })
                .map(from_ascii)
                .collect()
        } else {
            format!("HALTED WITH: {}", halted)
        }
    }
    fn solve_part_2(input: String) -> String {
        let mut computer = Computer::from(&input);
        // A B C D
        // if D
        // AND (!A || !B || !C)
        // NOT A T
        // NOT B J
        // OR T J J = !B || !A
        // NOT C T
        // OR T J
        // AND D J
        let commands = [
            "NOT C T", "AND D T", "AND H T", "OR T J", "NOT B T", "AND D T", "OR T J", "NOT A T",
            "OR T J",
        ];
        for ch in commands.join("\n").chars() {
            computer.add_input(to_ascii(ch));
        }
        for ch in "\nRUN\n".chars() {
            computer.add_input(to_ascii(ch));
        }
        let mut outputs = vec![];
        let halted = loop {
            match computer.run() {
                super::computer::Result::NeedsInput => panic!("ENOUGH INPUT ALREADY"),
                super::computer::Result::Output(n) => outputs.push(n),
                super::computer::Result::Halted(h) => break h,
            }
        };
        println!(
            "{}",
            outputs
                .iter()
                .map(|&n| n)
                .take_while(|&n| match char::from_u32(n as u32) {
                    Some(_) => true,
                    _ => false,
                })
                .map(from_ascii)
                .collect::<String>()
        );
        println!("{}", halted);
        if halted < (u32::MAX as i64) {
            outputs
                .into_iter()
                .take_while(|n| match char::from_u32(*n as u32) {
                    Some(_) => true,
                    _ => false,
                })
                .map(from_ascii)
                .collect()
        } else {
            format!("HALTED WITH: {}", halted)
        }
    }
}

#[cfg(test)]
mod day21_tests {
    use super::*;

    #[test]
    fn test_ascii() {
        assert_eq!(to_ascii('#'), 35);
        assert_eq!(to_ascii('A'), 65);
    }

    #[test]
    fn test_part_1() {
        let input = Day21::test_input();
        let ans = Day21::solve_part_1(input);
        assert_eq!(ans, "0")
    }

    #[test]
    fn test_part_2() {
        let input = Day21::test_input();
        let ans = Day21::solve_part_2(input);
        assert_eq!(ans, "0")
    }
}
