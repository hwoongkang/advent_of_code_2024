use super::Solution;

pub struct Day02;

fn is_safe(signal: &[i32]) -> bool {
    let mut prev = 0;
    let mut is_increasing = true;
    for (i, &num) in signal.iter().enumerate() {
        let diff = num - prev;
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
                let mut is_increasing = true;
                let mut prev = 0;
                for (i, str) in line.split_ascii_whitespace().enumerate() {
                    let num: i32 = str.parse().unwrap();
                    let diff = (num - prev).abs();

                    match i {
                        0 => {}
                        1 => {
                            is_increasing = num > prev;
                            if diff == 0 || diff > 3 {
                                return 0;
                            }
                        }
                        _ => {
                            if is_increasing != (num > prev) {
                                return 0;
                            } else if diff == 0 || diff > 3 {
                                return 0;
                            }
                        }
                    }
                    prev = num;
                }
                1
            })
            .sum::<u32>()
            .to_string()
    }

    fn solve_part_2(_input: String) -> String {
        let mut ans = 0;
        for line in _input.lines() {}
        String::from("0")
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
        assert_eq!(ans, "");
    }
}
