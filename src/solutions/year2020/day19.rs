use std::{cell::RefCell, collections::HashMap, rc::Rc};

use regex::Regex;

use crate::Solution;

#[derive(Debug)]
enum Rule {
    Literal(char),
    Concat(Vec<Rc<RefCell<Rule>>>),
    Or(Rc<RefCell<Rule>>, Rc<RefCell<Rule>>),
}

impl Rule {
    fn to_regex(&self) -> String {
        match self {
            Self::Literal(char) => format!("{}", char),
            Self::Concat(v) => v
                .iter()
                .map(|b| b.borrow().to_regex())
                .collect::<Vec<_>>()
                .join(""),

            Self::Or(left, right) => {
                format!(
                    "(?:{}|{})",
                    left.borrow().to_regex(),
                    right.borrow().to_regex()
                )
            }
        }
    }
}

#[derive(Debug)]
enum PreRule {
    Literal(char),
    Concat(Vec<i32>),
    Or(Rc<RefCell<PreRule>>, Rc<RefCell<PreRule>>),
}

impl PreRule {
    fn eval(&self, dict: &HashMap<i32, PreRule>) -> Rule {
        match self {
            Self::Literal(char) => Rule::Literal(*char),
            Self::Concat(v) => {
                let v = v
                    .iter()
                    .map(|i| dict.get(i).unwrap().eval(dict))
                    .map(|r| Rc::new(RefCell::new(r)))
                    .collect();
                Rule::Concat(v)
            }
            Self::Or(a, b) => {
                let left = Rc::new(RefCell::new(a.borrow().eval(dict)));
                let right = Rc::new(RefCell::new(b.borrow().eval(dict)));
                Rule::Or(left, right)
            }
        }
    }
}

fn parse_prerule(body: &str) -> PreRule {
    if body.starts_with("\"") {
        PreRule::Literal(body.chars().nth(1).unwrap())
    } else {
        let or: Vec<&str> = body.split(" | ").collect();
        if or.len() > 2 {
            panic!("WRONG INPUT");
        }
        if or.len() == 2 {
            PreRule::Or(
                Rc::new(RefCell::new(parse_prerule(or[0]))),
                Rc::new(RefCell::new(parse_prerule(or[1]))),
            )
        } else {
            let nums = body
                .split_ascii_whitespace()
                .map(|w| w.parse().unwrap())
                .collect();

            PreRule::Concat(nums)
        }
    }
}

fn parse_input(input: String) -> (HashMap<i32, PreRule>, Vec<String>) {
    let mut dict = HashMap::new();
    let mut lines = input.lines();
    for line in &mut lines {
        if line == "" {
            break;
        }
        let mut words = line.split(": ");
        let index: i32 = words.next().unwrap().parse().unwrap();
        let body = words.next().unwrap();

        let prerule = parse_prerule(body);
        dict.insert(index, prerule);
    }
    let lines = lines.map(|s| s.to_string()).collect();
    (dict, lines)
}

pub struct Day19 {}

impl Solution for Day19 {
    fn test_input() -> String {
        String::from(
            "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb",
        )
    }
    fn solve_part_1(input: String) -> String {
        let (dict, test_strings) = parse_input(input);
        let main_rule = dict.get(&0).unwrap().eval(&dict).to_regex();

        let reg = Regex::new(&format!("^{}$", main_rule)).unwrap();
        test_strings
            .iter()
            .filter(|line| reg.is_match(line))
            .count()
            .to_string()
    }
    fn solve_part_2(input: String) -> String {
        let (dict, test_strings) = parse_input(input);
        let mut regex_dict = HashMap::new();
        for (key, value) in dict.iter() {
            let regex = value.eval(&dict).to_regex();
            regex_dict.insert(*key, regex);
        }

        let forty_two = regex_dict.get(&42).unwrap();
        let thirty_one = regex_dict.get(&31).unwrap();

        let mut ans = 0;

        for line in test_strings {
            for i in 1..5 {
                let reg = format!(
                    "^({})+({}){{{}}}({}){{{}}}$",
                    forty_two, forty_two, i, thirty_one, i
                );
                let reg = Regex::new(&reg).unwrap();
                if reg.is_match(&line) {
                    ans += 1;
                    break;
                }
            }
        }

        ans.to_string()
    }
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day19::test_input();
        let ans = Day19::solve_part_1(input);
        assert_eq!(ans, "2")
    }

    #[test]
    fn test_part_2() {
        let input = String::from(
            "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        );
        let ans = Day19::solve_part_2(input);
        assert_eq!(ans, "12")
    }
}
