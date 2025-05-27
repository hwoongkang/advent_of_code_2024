use crate::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn test_input() -> String {
        String::from(
            "12
14
1969
100756",
        )
    }
    fn solve_part_1(input: String) -> String {
        let compute_fuel = |i: i32| -> i32 { i / 3 - 2 };
        input
            .lines()
            .map(|s| s.parse().unwrap())
            .map(|n| compute_fuel(n))
            .sum::<i32>()
            .to_string()
    }
    fn solve_part_2(input: String) -> String {
        let compute_fuel = |mut i: i32| -> i32 {
            let mut sum = 0;
            loop {
                i = i / 3 - 2;
                if i <= 0 {
                    break;
                }
                sum += i;
            }
            sum
        };
        input
            .lines()
            .map(|s| s.parse().unwrap())
            .map(|n| compute_fuel(n))
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod day01_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day01::test_input();
        let ans = Day01::solve_part_1(input);
        assert_eq!(ans, (2 + 2 + 654 + 33583).to_string())
    }

    #[test]
    fn test_part_2() {
        let input = Day01::test_input();
        let ans = Day01::solve_part_2(input);
        assert_eq!(ans, (2 + 2 + 966 + 50346).to_string())
    }
}
