use crate::Solution;

pub struct Day07;

impl Solution for Day07 {
    fn test_input() -> String {
        String::from(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        )
    }

    fn solve_part_1(_input: String) -> String {
        fn backtrack(target: usize, now: usize, nums: &[usize]) -> bool {
            if now > target {
                false
            } else if nums.len() == 0 {
                now == target
            } else {
                backtrack(target, now + nums[0], &nums[1..])
                    || backtrack(target, now * nums[0], &nums[1..])
            }
        }
        let mut total = 0;

        for line in _input.lines() {
            let mut words = line.split(":");
            let ans: usize = words.next().unwrap().parse().unwrap();
            let nums: Vec<usize> = words
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|w| w.parse().unwrap())
                .collect();
            if backtrack(ans, nums[0], &nums[1..]) {
                total += ans;
            }
        }

        total.to_string()
    }

    fn solve_part_2(_input: String) -> String {
        fn concat(mut a: usize, b: usize) -> usize {
            let mut n = b;
            while n > 0 {
                a *= 10;
                n /= 10;
            }
            a + b
        }

        fn backtrack(target: usize, now: usize, nums: &[usize]) -> bool {
            if now > target {
                false
            } else if nums.len() == 0 {
                now == target
            } else {
                backtrack(target, now + nums[0], &nums[1..])
                    || backtrack(target, now * nums[0], &nums[1..])
                    || backtrack(target, concat(now, nums[0]), &nums[1..])
            }
        }
        let mut total = 0;

        for line in _input.lines() {
            let mut words = line.split(":");
            let ans: usize = words.next().unwrap().parse().unwrap();
            let nums: Vec<usize> = words
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|w| w.parse().unwrap())
                .collect();
            if backtrack(ans, nums[0], &nums[1..]) {
                total += ans;
            }
        }

        total.to_string()
    }
}

#[cfg(test)]
mod day07_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day07::test_input();
        let ans = Day07::solve_part_1(input);
        assert_eq!(ans, "3749");
    }

    #[test]
    fn test_part_2() {
        let input = Day07::test_input();
        let ans = Day07::solve_part_2(input);
        assert_eq!(ans, "11387");
    }
}
