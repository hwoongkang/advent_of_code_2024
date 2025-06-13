use crate::Solution;

#[derive(PartialEq, Eq, Debug)]
enum Shuffle {
    Cut(i32),
    NewStack,
    Increment(usize),
}

impl Shuffle {
    fn from(line: &str) -> Self {
        if line.starts_with("deal") {
            if line.ends_with("stack") {
                Self::NewStack
            } else {
                let num = line
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();
                Self::Increment(num)
            }
        } else {
            let num = line
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            Self::Cut(num)
        }
    }

    fn apply(&self, mut v: Vec<i32>) -> Vec<i32> {
        match &self {
            Self::NewStack => {
                v.reverse();
                v
            }
            Self::Cut(i) => {
                let i = if *i > 0 {
                    *i as usize
                } else {
                    v.len() - (-i as usize)
                };
                v.rotate_left(i);
                v
            }
            Self::Increment(i) => {
                let l = v.len();
                let mut new = vec![0; l];
                for index in 0..v.len() {
                    let new_index = (index * i) % l;
                    new[new_index] = v[index];
                }
                new
            }
        }
    }
}

pub struct Day22 {}

impl Solution for Day22 {
    fn test_input() -> String {
        String::new()
    }
    fn solve_part_1(input: String) -> String {
        let mut v = (0..10_007).collect();
        for line in input.lines() {
            let cmd = Shuffle::from(line);
            v = cmd.apply(v);
        }
        v.iter()
            .enumerate()
            .find(|(_, v)| **v == 2019)
            .unwrap()
            .0
            .to_string()
    }
    fn solve_part_2(_input: String) -> String {
        String::new()
    }
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = String::from(
            "cut 6
deal with increment 7
deal into new stack
cut -2",
        );
        let cmds: Vec<Shuffle> = input.lines().map(Shuffle::from).collect();
        assert_eq!(
            cmds,
            vec![
                Shuffle::Cut(6),
                Shuffle::Increment(7),
                Shuffle::NewStack,
                Shuffle::Cut(-2)
            ]
        )
    }

    #[test]
    fn test_cmds() {
        let v: Vec<i32> = (0..10).collect();
        assert_eq!(
            Shuffle::NewStack.apply(v.clone()),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        );
        assert_eq!(
            Shuffle::Cut(3).apply(v.clone()),
            vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]
        );
        assert_eq!(
            Shuffle::Cut(-4).apply(v.clone()),
            vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5,]
        );
        assert_eq!(
            Shuffle::Increment(3).apply(v.clone()),
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]
        );
    }

    #[test]
    fn test_part_1() {
        let input = Day22::test_input();
        let ans = Day22::solve_part_1(input);
        assert_eq!(ans, "0")
    }

    #[test]
    fn test_part_2() {
        let input = Day22::test_input();
        let ans = Day22::solve_part_2(input);
        assert_eq!(ans, "0")
    }
}
