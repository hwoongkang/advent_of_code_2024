use super::Solution;
use regex::Regex;

pub struct Day03;

impl Solution for Day03 {
    fn test_input() -> String {
        String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
    }

    fn solve_part_1(_input: String) -> String {
        let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
        let mut ans = 0;

        for re_match in re.captures_iter(&_input) {
            let (_, [n, m]) = re_match.extract();
            let n: usize = n.parse().unwrap();
            let m: usize = m.parse().unwrap();
            ans += n * m;
        }
        ans.to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod day03_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day03::test_input();
        let ans = Day03::solve_part_1(input);
        assert_eq!(ans, "161");
    }

    #[test]
    fn test_part_2() {
        let input = Day03::test_input();
        let ans = Day03::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
