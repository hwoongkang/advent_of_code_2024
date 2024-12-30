use crate::Solution;

#[derive(PartialEq, Eq)]
enum Lock {
    Lock(u64),
    Key(u64),
}

impl Lock {
    fn from(lines: &mut std::str::Lines) -> Option<Self> {
        let line = lines.next();
        let Some(line) = line else {
            return None;
        };
        let is_key = line.starts_with("..");
        let mut num = 0;
        for (r, line) in lines.take(5).enumerate() {
            for (c, ch) in line.chars().enumerate() {
                let index = 5 * (4 - c) + (4 - r);
                let n = if ch == '.' { 0 } else { 1 };
                num += n << index;
            }
        }
        lines.next();
        lines.next();
        Some(if is_key {
            Self::Key(num)
        } else {
            Self::Lock(num)
        })
    }

    fn matches_part_1(&self, rhs: &Self) -> bool {
        let a;
        let b;
        match (self, rhs) {
            (Self::Key(_), Self::Key(_)) => return false,
            (Self::Lock(_), Self::Lock(_)) => return false,
            (Self::Key(n), Self::Lock(m)) => {
                a = *n;
                b = *m;
            }
            (Self::Lock(m), Self::Key(n)) => {
                a = *n;
                b = *m;
            }
        }
        a & b == 0
    }
}

pub struct Day25;

impl Solution for Day25 {
    fn test_input() -> String {
        String::from(
            "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####",
        )
    }

    fn solve_part_1(input: String) -> String {
        let mut locks: Vec<Lock> = vec![];
        let mut lines = input.lines();
        while let Some(lock) = Lock::from(&mut lines) {
            locks.push(lock);
        }
        let mut ans = 0;

        for i in 0..locks.len() {
            for j in i + 1..locks.len() {
                let lhs = &locks[i];
                let rhs = &locks[j];
                if lhs.matches_part_1(rhs) {
                    ans += 1;
                }
            }
        }
        ans.to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("Merry Christmas")
    }
}

#[cfg(test)]
mod day25_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day25::test_input();
        let ans = Day25::solve_part_1(input);
        assert_eq!(ans, "3");
    }

    #[test]
    fn test_part_2() {
        let input = Day25::test_input();
        let ans = Day25::solve_part_2(input);
        assert_eq!(ans, "Merry Christmas");
    }
}
