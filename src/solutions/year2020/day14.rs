use std::collections::HashMap;

use crate::Solution;

struct Mask([u8; 36]);

impl Mask {
    fn from(line: &str) -> Self {
        let mut data = [0; 36];
        for (ind, char) in line[7..].chars().rev().enumerate() {
            data[ind] = match char {
                'X' => 2,
                '1' => 1,
                '0' => 0,
                _ => panic!("Wrong input"),
            }
        }
        Self(data)
    }

    fn process(&self, mut num: i64) -> i64 {
        let mut ans = 0;
        for ind in 0..36 {
            let bit = match &self.0[ind] {
                0 => 0,
                1 => 1,
                _ => num & 1,
            };
            ans |= bit << ind;
            num >>= 1;
        }
        ans
    }

    fn evaluate(&self, mem: usize) -> Vec<usize> {
        let mut nums = vec![0];
        for ind in 0..36 {
            let flag = &self.0[ind];
            match flag {
                0 => nums = nums.into_iter().map(|n| n | (mem & (1 << ind))).collect(),
                1 => nums = nums.into_iter().map(|n| n | (1 << ind)).collect(),
                _ => {
                    let mut new_nums = vec![];
                    for num in nums {
                        new_nums.push(num | (1 << ind));
                        new_nums.push(num);
                    }
                    nums = new_nums;
                }
            }
        }
        nums
    }
}

enum Cmd {
    Mask(Mask),
    Mem(usize, i64),
}

impl Cmd {
    fn from(line: &str) -> Self {
        let is_mask = line.starts_with("ma");
        if is_mask {
            Self::Mask(Mask::from(line))
        } else {
            let mut words = line.split_ascii_whitespace();
            let mem = words.next().unwrap();
            let num = words.nth(1).unwrap().parse().unwrap();

            let l = mem.len();
            let mem = mem[4..l - 1].parse().unwrap();
            Self::Mem(mem, num)
        }
    }
}

pub struct Day14 {}

impl Solution for Day14 {
    fn test_input() -> String {
        String::from(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0",
        )
    }
    fn solve_part_1(input: String) -> String {
        let mut memory: HashMap<usize, i64> = HashMap::new();
        let mut mask = Mask([0; 36]);
        for line in input.lines() {
            match Cmd::from(line) {
                Cmd::Mask(m) => {
                    mask = m;
                }
                Cmd::Mem(mem, num) => {
                    memory.insert(mem, mask.process(num));
                }
            }
        }
        let sum = memory.values().sum::<i64>();
        sum.to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut memory: HashMap<usize, i64> = HashMap::new();
        let mut mask = Mask([0; 36]);
        for line in input.lines() {
            match Cmd::from(line) {
                Cmd::Mask(m) => {
                    mask = m;
                }
                Cmd::Mem(mem, num) => {
                    for mem in mask.evaluate(mem) {
                        memory.insert(mem, num);
                    }
                }
            }
        }
        let sum = memory.values().sum::<i64>();
        sum.to_string()
    }
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    #[test]
    fn test_eval() {
        let mask = Mask::from("mask = 000000000000000000000000000000X1001X");
        let mut result = mask.evaluate(42);
        result.sort();
        assert_eq!(result, [26, 27, 58, 59]);
    }

    #[test]
    fn test_mask() {
        let mask = Mask::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(mask.process(11), 73);
        assert_eq!(mask.process(101), 101);
        assert_eq!(mask.process(0), 64);
    }

    #[test]
    fn test_part_1() {
        let input = Day14::test_input();
        let ans = Day14::solve_part_1(input);
        assert_eq!(ans, "165")
    }

    #[test]
    fn test_part_2() {
        let input = String::from(
            "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1",
        );
        let ans = Day14::solve_part_2(input);
        assert_eq!(ans, "208")
    }
}
