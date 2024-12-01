use std::collections::HashMap;

use super::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn test_input() -> String {
        String::from(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        )
    }

    fn solve_part_1(_input: String) -> String {
        let mut v1: Vec<usize> = vec![];
        let mut v2: Vec<usize> = vec![];
        for line in _input.lines() {
            let mut words = line.split("   ");
            v1.push(words.next().unwrap().parse().unwrap());
            v2.push(words.next().unwrap().parse().unwrap());
        }
        v1.sort();
        v2.sort();
        v1.into_iter()
            .zip(v2.into_iter())
            .map(|(n, m)| if n > m { n - m } else { m - n })
            .sum::<usize>()
            .to_string()
    }

    fn solve_part_2(_input: String) -> String {
        let mut v1: Vec<usize> = vec![];
        let mut v2: HashMap<usize, usize> = HashMap::new();
        for line in _input.lines() {
            let mut words = line.split("   ");
            v1.push(words.next().unwrap().parse().unwrap());
            let key = words.next().unwrap().parse().unwrap();
            let val = v2.entry(key).or_insert(0);
            *val += 1;
        }
        v1.into_iter()
            .map(|key| key * v2.get(&key).unwrap_or(&0))
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod day01_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day01::test_input();
        let ans = Day01::solve_part_1(input);
        assert_eq!(ans, "11");
    }

    #[test]
    fn test_part_2() {
        let input = Day01::test_input();
        let ans = Day01::solve_part_2(input);
        assert_eq!(ans, "31");
    }
}
