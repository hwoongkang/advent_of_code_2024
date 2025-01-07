use crate::Solution;

pub struct Day11 {}

impl Solution for Day11 {
    fn test_input() -> String {
        String::from(
            "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3",
        )
    }

    fn solve_part_1(input: String) -> String {
        let mut adapters: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
        adapters.sort();
        let max = adapters.last().unwrap() + 3;
        let mut with_outlet = vec![0];
        with_outlet.append(&mut adapters);
        with_outlet.push(max);

        let mut ones = 0;
        let mut threes = 0;
        for (i, j) in with_outlet.iter().zip(with_outlet.iter().skip(1)) {
            let diff = j - i;
            match diff {
                1 => ones += 1,
                3 => threes += 1,
                _ => {}
            }
        }
        (ones * threes).to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut adapters: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
        adapters.sort();
        let max = adapters.last().unwrap() + 3;
        let mut with_outlet = vec![0];
        with_outlet.append(&mut adapters);
        with_outlet.push(max);

        let max = max as usize;

        let mut dp = vec![0usize; max + 1];

        dp[0] = 1;

        for &num in with_outlet.iter().skip(1) {
            let num = num as usize;
            for i in num.saturating_sub(3)..num {
                dp[num] += dp[i];
            }
        }

        dp[max].to_string()
    }
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day11::test_input();
        let ans = Day11::solve_part_1(input);
        assert_eq!(ans, "220")
    }

    #[test]
    fn test_part_2() {
        let input = Day11::test_input();
        let ans = Day11::solve_part_2(input);
        assert_eq!(ans, "19208")
    }
}
