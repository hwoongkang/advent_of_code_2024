use super::Solution;
use regex::Regex;

pub struct Day03;

fn eval_mul(cmd: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut ans = 0;

    for re_match in re.captures_iter(cmd) {
        let (_, [n, m]) = re_match.extract();
        let n: usize = n.parse().unwrap();
        let m: usize = m.parse().unwrap();
        ans += n * m;
    }
    ans
}

impl Solution for Day03 {
    fn test_input() -> String {
        String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
    }

    fn solve_part_1(_input: String) -> String {
        eval_mul(&_input).to_string()
    }

    fn solve_part_2(_input: String) -> String {
        let chunks: Vec<&str> = _input.split("don't()").collect();

        let mut ans = 0;
        for (i, chunk) in chunks.into_iter().enumerate() {
            let should_do = i == 0;
            let sub_chunks = chunk.split("do()");
            let start_index = if should_do { 0 } else { 1 };
            for sub_chunk in sub_chunks.skip(start_index) {
                ans += eval_mul(sub_chunk);
            }
        }
        ans.to_string()
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
        assert_eq!(ans, "48");
    }
}
