use std::collections::HashMap;

type Database = HashMap<usize, usize>;

use crate::Solution;

fn eval(database: &mut Database, turn: usize, num: usize) -> usize {
    if let Some(prev_turn) = database.insert(num, turn) {
        turn - prev_turn
    } else {
        0
    }
}

pub struct Day15 {}

impl Solution for Day15 {
    fn test_input() -> String {
        String::from("0,3,6")
    }
    fn solve_part_1(input: String) -> String {
        let mut database = Database::new();
        let starting: Vec<usize> = input.split(",").map(|n| n.parse().unwrap()).collect();
        let l = starting.len();
        let mut result = starting[l - 1];
        for (ind, n) in starting.into_iter().enumerate() {
            result = eval(&mut database, ind, n);
        }

        for turn in l..2019 {
            result = eval(&mut database, turn, result);
        }

        result.to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut database = Database::new();
        let starting: Vec<usize> = input.split(",").map(|n| n.parse().unwrap()).collect();
        let l = starting.len();
        let mut result = starting[l - 1];
        for (ind, n) in starting.into_iter().enumerate() {
            result = eval(&mut database, ind, n);
        }

        for turn in l..30000000 - 1 {
            result = eval(&mut database, turn, result);
        }

        result.to_string()
    }
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day15::test_input();
        let ans = Day15::solve_part_1(input);
        assert_eq!(ans, "436")
    }

    #[test]
    fn test_part_2() {
        let input = Day15::test_input();
        let ans = Day15::solve_part_2(input);
        assert_eq!(ans, "0")
    }
}
