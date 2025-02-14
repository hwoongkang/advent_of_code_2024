use crate::Solution;

pub struct Day18 {}

impl Solution for Day18 {
    fn test_input() -> String {
        String::from(
            "2 * 3 + (4 * 5)
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
        )
    }
    fn solve_part_1(input: String) -> String {
        input.lines().map(eval).sum::<i64>().to_string()
    }
    fn solve_part_2(input: String) -> String {
        input.lines().map(eval_2).sum::<i64>().to_string()
    }
}

fn matching_pairs(line: &str) -> Vec<(usize, usize)> {
    let mut pairs = vec![];

    let mut start = 0;

    let mut depth = 0;

    for (ind, char) in line.chars().enumerate() {
        match char {
            '(' => {
                if depth == 0 {
                    start = ind;
                }
                depth += 1;
            }
            ')' => {
                depth -= 1;
                if depth == 0 {
                    pairs.push((start, ind));
                }
            }
            _ => {}
        }
    }

    pairs
}

#[derive(PartialEq, Eq, Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(PartialEq, Eq, Debug)]
enum Token {
    Digit(i64),
    Op(Op),
}

fn eval_pure_2(line: &str) -> i64 {
    let tokens: Vec<Token> = line
        .split_ascii_whitespace()
        .map(|w| match w {
            "+" => Token::Op(Op::Add),
            "*" => Token::Op(Op::Mul),
            d => Token::Digit(d.parse().unwrap()),
        })
        .collect();
    let mut ans = 1;
    for sub_tokens in tokens.split(|t| t == &Token::Op(Op::Mul)) {
        let mut value = 0;
        for token in sub_tokens {
            match token {
                Token::Digit(d) => value += d,
                _ => {}
            }
        }
        ans *= value;
    }
    ans
}

fn eval_pure(line: &str) -> i64 {
    let mut value: Option<i64> = None;

    let mut op: Op = Op::Add;

    for token in line.split_ascii_whitespace() {
        match token {
            "+" => op = Op::Add,
            "*" => op = Op::Mul,
            d => {
                let n = d.parse().unwrap();
                if let Some(acc) = &mut value {
                    match op {
                        Op::Add => *acc += n,
                        Op::Mul => *acc *= n,
                    }
                } else {
                    value = Some(n);
                }
            }
        }
    }
    if let Some(value) = value {
        value
    } else {
        panic!("Invalid expression")
    }
}

fn eval(expr: &str) -> i64 {
    let nested = matching_pairs(expr);
    if nested.len() == 0 {
        return eval_pure(expr);
    }
    let mut new_line = String::new();

    let mut prev = 0;
    for (s, e) in nested {
        new_line += &expr[prev..s];

        prev = e + 1;
        new_line += &eval(&expr[s + 1..e]).to_string();
    }
    new_line += &expr[prev..];

    eval(&new_line)
}

fn eval_2(expr: &str) -> i64 {
    let nested = matching_pairs(expr);
    if nested.len() == 0 {
        return eval_pure_2(expr);
    }
    let mut new_line = String::new();

    let mut prev = 0;
    for (s, e) in nested {
        new_line += &expr[prev..s];

        prev = e + 1;
        new_line += &eval_2(&expr[s + 1..e]).to_string();
    }
    new_line += &expr[prev..];

    eval_2(&new_line)
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    #[test]
    fn test_pairs() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))".to_string();
        assert_eq!(matching_pairs(&input), [(4, 10), (14, 26)]);
    }

    #[test]
    fn test_pure() {
        let input = "1 + 2 * 3 + 4 * 5 + 6".to_string();
        assert_eq!(Day18::solve_part_1(input), "71")
    }

    #[test]
    fn test_pure_2() {
        let input = "1 + 2 * 3 + 4 * 5 + 6".to_string();
        assert_eq!(Day18::solve_part_2(input), "231")
    }

    #[test]
    fn test_part_1() {
        let input = Day18::test_input();
        let ans = Day18::solve_part_1(input);
        assert_eq!(ans, "13658")
    }

    #[test]
    fn test_part_2() {
        let input = Day18::test_input();
        let ans = Day18::solve_part_2(input);
        let desired = 46 + 23340;
        assert_eq!(ans, desired.to_string())
    }
}
