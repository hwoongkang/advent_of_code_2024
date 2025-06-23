use crate::{solutions::year2019::computer, Solution};

use super::computer::Computer;

pub struct Day23 {}

#[derive(Clone)]
struct Packet(i64, i64);

struct Network {
    computers: Vec<Computer>,
    packets: Vec<Vec<Packet>>,
}

impl Network {
    fn new(input: String, n: i64) -> Self {
        Self {
            computers: (0..n)
                .map(|i| {
                    let mut computer = Computer::from(&input);
                    computer.add_input(i);
                    computer.add_input(-1);
                    computer
                })
                .collect(),
            packets: vec![vec![]; n as usize],
        }
    }

    fn boot(&mut self) {
        for (i, com) in self.computers.iter_mut().enumerate() {
            print!("Computer #{} is ", i + 1);
            match com.run() {
                computer::Result::NeedsInput => {
                    println!("receiving a packet")
                }
                computer::Result::Output(i) => {
                    let computer::Result::Output(x) = com.run() else {
                        panic!("OUTPUT WRONG")
                    };
                    let computer::Result::Output(y) = com.run() else {
                        panic!("OUTPUT WRONG")
                    };
                    println!("sending a packet({},{}) to {}", x, y, i);
                }
                computer::Result::Halted(i) => {
                    println!("halting with {}", i)
                }
            }
        }
    }
}

impl Solution for Day23 {
    fn test_input() -> String {
        String::new()
    }
    fn solve_part_1(input: String) -> String {
        let mut network = Network::new(input, 50);
        network.boot();
        String::new()
    }
    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

#[cfg(test)]
mod day23_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day23::test_input();
        let ans = Day23::solve_part_1(input);
        assert_eq!(ans, "0")
    }

    #[test]
    fn test_part_2() {
        let input = Day23::test_input();
        let ans = Day23::solve_part_2(input);
        assert_eq!(ans, "0")
    }
}
