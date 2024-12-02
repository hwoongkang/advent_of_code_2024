use super::Solution;

pub struct Day02;

fn is_safe(signal: &[i64]) -> bool {
    let mut prev = 0;
    let mut is_increasing = true;
    for (i, &num) in signal.iter().enumerate() {
        let diff = (num - prev).abs();
        let is_diff_safe = 1 <= diff && diff <= 3;
        match i {
            0 => {}
            1 => {
                is_increasing = num > prev;
                if !is_diff_safe {
                    return false;
                }
            }
            _ => {
                if (num > prev) != is_increasing || !is_diff_safe {
                    return false;
                }
            }
        }
        prev = num
    }
    true
}

impl Solution for Day02 {
    fn test_input() -> String {
        String::from(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        )
    }

    fn solve_part_1(_input: String) -> String {
        _input
            .lines()
            .map(|line| {
                let signal: Vec<i64> = line
                    .split_ascii_whitespace()
                    .map(|ch| ch.parse().unwrap())
                    .collect();
                if is_safe(&signal) {
                    1
                } else {
                    0
                }
            })
            .sum::<u32>()
            .to_string()
    }

    fn solve_part_2(_input: String) -> String {
        let mut ans = 0;
        'line: for line in _input.lines() {
            let signal: Vec<i64> = line
                .split_ascii_whitespace()
                .map(|ch| ch.parse().unwrap())
                .collect();
            if is_safe(&signal) {
                ans += 1;
                continue;
            }
            let len = signal.len();
            for i in 0..len {
                let mut s = signal.clone();
                s.remove(i);
                if is_safe(&s) {
                    ans += 1;
                    continue 'line;
                }
            }
        }
        ans.to_string()
    }
}

#[cfg(test)]
mod day02_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day02::test_input();
        let ans = Day02::solve_part_1(input);
        assert_eq!(ans, "2");
    }

    #[test]
    fn test_part_2() {
        let input = Day02::test_input();
        let ans = Day02::solve_part_2(input);
        assert_eq!(ans, "4");
    }
}
