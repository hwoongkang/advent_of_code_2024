use std::collections::VecDeque;

use crate::Solution;

pub struct Day06 {}

impl Solution for Day06 {
    fn test_input() -> String {
        String::from(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9",
        )
    }
    fn solve_part_1(input: String) -> String {
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;
        let mut coords: Vec<(i32, i32)> = input
            .lines()
            .map(|line| {
                let mut words = line.split(", ");
                let x = words.next().unwrap().parse().unwrap();
                let y = words.next().unwrap().parse().unwrap();
                x_min = x_min.min(x);
                y_min = y_min.min(y);
                x_max = x_max.max(x);
                y_max = y_max.max(y);
                (x, y)
            })
            .collect();
        let x_span = x_max - x_min;
        let y_span = y_max - y_min;
        coords = coords
            .into_iter()
            .map(|(x, y)| (x - x_min + x_span, y - y_min + y_span))
            .collect();

        let x_max = 3 * x_span;
        let y_max = 3 * y_span;

        let mx = x_max as usize;
        let my = y_max as usize;

        let mut visited: Vec<Vec<(i32, Vec<usize>)>> = vec![vec![(-1, vec![]); my]; mx];

        let mut queue: VecDeque<(usize, (usize, usize), i32)> = coords
            .into_iter()
            .enumerate()
            .map(|(id, pos)| (id, (pos.0 as usize, pos.1 as usize), 0))
            .collect();

        while let Some((id, pos, dist)) = queue.pop_front() {}

        String::new()
    }
    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day06::test_input();
        let ans = Day06::solve_part_1(input);
        assert_eq!(ans, "0")
    }

    #[test]
    fn test_part_2() {
        let input = Day06::test_input();
        let ans = Day06::solve_part_2(input);
        assert_eq!(ans, "0")
    }
}
