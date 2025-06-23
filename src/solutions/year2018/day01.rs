use std::collections::HashSet;

use crate::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn test_input() -> String {
        String::new()
    }
    fn solve_part_1(input: String) -> String {
        input
            .lines()
            .map(|line| {
                let num: i32 = line[1..].parse().unwrap();
                let plus = line.starts_with("+");
                if plus {
                    num
                } else {
                    -num
                }
            })
            .sum::<i32>()
            .to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut sum = 0;
        let mut seen = HashSet::from([0]);
        for line in input.lines().cycle() {
            let num: i32 = line[1..].parse().unwrap();
            let plus = line.starts_with("+");
            let num = if plus { num } else { -num };
            sum += num;
            if !seen.insert(sum) {
                return sum.to_string();
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod day01_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = String::from("+1\n+1\n+1");
        let ans = Day01::solve_part_1(input);
        assert_eq!(ans, "3");
        let input = String::from("+1\n+1\n-2");
        let ans = Day01::solve_part_1(input);
        assert_eq!(ans, "0");
        let input = String::from("-1\n-2\n-3");
        let ans = Day01::solve_part_1(input);
        assert_eq!(ans, "-6");
    }

    #[test]
    fn test_part_2() {
        let input = String::from("+1\n-2\n+3\n+1");
        let ans = Day01::solve_part_2(input);
        assert_eq!(ans, "2");
        let input = String::from("+1\n-1");
        let ans = Day01::solve_part_2(input);
        assert_eq!(ans, "0");
        let input = String::from("+3\n+3\n+4\n-2\n-4");
        let ans = Day01::solve_part_2(input);
        assert_eq!(ans, "10");
        let input = String::from("-6\n+3\n+8\n+5\n-6");
        let ans = Day01::solve_part_2(input);
        assert_eq!(ans, "5");
        let input = String::from("+7\n+7\n-2\n-7\n-4");
        let ans = Day01::solve_part_2(input);
        assert_eq!(ans, "14");
    }
}
