use crate::Solution;

pub struct Day05 {}

#[derive(Debug)]
enum Unit {
    Upper(u32),
    Lower(u32),
}

impl Unit {
    fn from(char: char) -> Self {
        if char.is_ascii_lowercase() {
            Self::Lower(char as u32 - 'a' as u32)
        } else {
            Self::Upper(char as u32 - 'A' as u32)
        }
    }

    fn matches(&self, other: &Self) -> bool {
        use Unit::*;
        match (self, other) {
            (Upper(_), Upper(_)) | (Lower(_), Lower(_)) => false,
            (Lower(i), Upper(j)) | (Upper(i), Lower(j)) => i == j,
        }
    }

    fn id(&self) -> u32 {
        match self {
            Self::Upper(i) => *i,
            Self::Lower(i) => *i,
        }
    }
}

impl Solution for Day05 {
    fn test_input() -> String {
        String::from("dabAcCaCBAcCcaDA")
    }
    fn solve_part_1(input: String) -> String {
        let mut stack: Vec<Unit> = vec![];
        for unit in input.chars().map(Unit::from) {
            if let Some(last) = stack.pop() {
                if !last.matches(&unit) {
                    stack.push(last);
                    stack.push(unit);
                }
            } else {
                stack.push(unit);
            }
        }
        stack.len().to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut ans = usize::MAX;
        for skip in 0..26 {
            let mut stack: Vec<Unit> = vec![];
            for unit in input
                .chars()
                .map(Unit::from)
                .filter(|unit| unit.id() != skip)
            {
                if let Some(last) = stack.pop() {
                    if !last.matches(&unit) {
                        stack.push(last);
                        stack.push(unit);
                    }
                } else {
                    stack.push(unit);
                }
            }
            ans = ans.min(stack.len());
        }

        ans.to_string()
    }
}

#[cfg(test)]
mod day05_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day05::test_input();
        let ans = Day05::solve_part_1(input);
        assert_eq!(ans, "10")
    }

    #[test]
    fn test_part_2() {
        let input = Day05::test_input();
        let ans = Day05::solve_part_2(input);
        assert_eq!(ans, "4")
    }
}
