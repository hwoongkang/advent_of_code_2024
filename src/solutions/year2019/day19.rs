use crate::Solution;

use super::computer::{self, Computer};

pub struct Day19 {}

fn check(input: &String, x: i64, y: i64) -> bool {
    let mut computer = Computer::from(input);
    computer.input_seq = vec![x, y];
    match computer.run() {
        computer::Result::NeedsInput => panic!("Shouldn't need more input"),
        computer::Result::Halted(_) => panic!("I am waiting for an input"),
        computer::Result::Output(i) => return i == 1,
    }
}

impl Solution for Day19 {
    fn test_input() -> String {
        String::new()
    }
    fn solve_part_1(input: String) -> String {
        let mut ans = 0;
        for x in 0..50 {
            for y in 0..50 {
                if check(&input, x, y) {
                    ans += 1;
                }
            }
        }
        ans.to_string()
    }
    fn solve_part_2(input: String) -> String {
        for d in 0.. {
            let mut start = -1;
            for i in (0..=d).rev() {
                let affected = check(&input, d, i);
                if affected && start == -1 {
                    start = i;
                }
                if i < d - 100 && !affected {
                    break;
                }
                if affected && start - i == 99 {
                    let row = d;
                    let col = i;
                    if (1..100).all(|dr| check(&input, row + dr, col)) {
                        let mut board = vec![vec!['.'; 100]; 100];
                        for dr in 0..100 {
                            for dc in 0..100 {
                                if check(&input, row + dr, col + dc) {
                                    board[dr as usize][dc as usize] = '#';
                                }
                            }
                        }

                        return (row * 10_000 + col).to_string();
                    }
                }
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day19::test_input();
        let ans = Day19::solve_part_1(input);
        assert_eq!(ans, "0")
    }

    #[test]
    fn test_part_2() {
        let input = Day19::test_input();
        let ans = Day19::solve_part_2(input);
        assert_eq!(ans, "0")
    }
}
