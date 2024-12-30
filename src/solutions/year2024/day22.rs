use std::collections::{HashMap, VecDeque};

use crate::Solution;

const PRUNE: i64 = 16777216;

fn mix(parent: i64, child: i64) -> i64 {
    parent ^ child
}

fn prune(parent: i64) -> i64 {
    parent % PRUNE
}

fn next(mut num: i64) -> i64 {
    num = mix(num, num * 64);
    num = prune(num);
    num = mix(num, num / 32);
    num = prune(num);
    num = mix(num, num * 2048);
    num = prune(num);
    num
}

pub struct Day22;

impl Solution for Day22 {
    fn test_input() -> String {
        String::from(
            "1
10
100
2024",
        )
    }

    fn solve_part_1(input: String) -> String {
        let mut ans = 0;
        for line in input.lines() {
            let mut num = line.parse().unwrap();
            for _ in 0..2000 {
                num = next(num);
            }
            ans += num;
        }
        ans.to_string()
    }

    fn solve_part_2(input: String) -> String {
        let mut total: HashMap<[i64; 4], i64> = HashMap::new();
        for line in input.lines() {
            let mut local: HashMap<[i64; 4], i64> = HashMap::new();
            let mut queue = VecDeque::new();
            let mut num = line.parse().unwrap();

            for _ in 0..2000 {
                let new_num = next(num);
                let diff = new_num % 10 - num % 10;
                queue.push_back(diff);
                if queue.len() > 4 {
                    queue.pop_front();
                }

                num = new_num;
                if queue.len() == 4 {
                    let key = [queue[0], queue[1], queue[2], queue[3]];
                    local.entry(key).or_insert(num % 10);
                }
            }
            for (key, value) in local.into_iter() {
                let entry = total.entry(key).or_insert(0);
                *entry += value;
            }
        }

        total.into_values().max().unwrap().to_string()
    }
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    #[test]
    fn test_pseudo_random() {
        let mut num = 123;
        let ans = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        let mut sq = vec![];
        for _ in 0..10 {
            num = next(num);
            sq.push(num);
        }
        assert_eq!(sq, ans);
    }

    #[test]
    fn test_part_1() {
        let input = Day22::test_input();
        let ans = Day22::solve_part_1(input);
        assert_eq!(ans, "37327623");
    }

    #[test]
    fn test_part_2() {
        let input = "1
2
3
2024"
            .to_string();
        let ans = Day22::solve_part_2(input);
        assert_eq!(ans, "23");
    }
}
