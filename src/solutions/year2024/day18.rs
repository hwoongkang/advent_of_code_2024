use std::collections::VecDeque;

use crate::Solution;

pub struct Day18;

fn simulate(map: &mut Vec<Vec<bool>>, block: (usize, usize)) -> bool {
    let map_size = map.len();
    let mut queue = VecDeque::from([(0, 0)]);

    let end = (map_size - 1, map_size - 1);

    let mut visited = vec![vec![false; map_size]; map_size];
    visited[0][0] = true;

    let (r, c) = block;
    map[r][c] = false;

    while let Some((r, c)) = queue.pop_front() {
        if (r, c) == end {
            return true;
        }
        let ir = r as i32;
        let ic = c as i32;
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let r = ir + dr;
            let c = ic + dc;
            if r < 0 || c < dc {
                continue;
            }
            let r = r as usize;
            let c = c as usize;
            if r >= map_size || c >= map_size {
                continue;
            }
            if !map[r][c] {
                continue;
            }
            if visited[r][c] {
                continue;
            }
            visited[r][c] = true;
            queue.push_back((r, c))
        }
    }
    false
}

impl Solution for Day18 {
    fn test_input() -> String {
        String::from(
            "7
12
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
        )
    }

    fn solve_part_1(input: String) -> String {
        let lines = &mut input.lines();
        let map_size: usize = lines.next().unwrap().parse().unwrap();
        let mut map: Vec<Vec<bool>> = vec![vec![true; map_size]; map_size];
        let bytes: usize = lines.next().unwrap().parse().unwrap();
        for _ in 0..bytes {
            let mut words = lines.next().unwrap().split(",");
            let r: usize = words.next().unwrap().parse().unwrap();
            let c: usize = words.next().unwrap().parse().unwrap();
            map[r][c] = false;
        }

        let mut queue = VecDeque::from([(0, 0, 0)]);

        let end = (map_size - 1, map_size - 1);

        let mut visited = vec![vec![false; map_size]; map_size];
        visited[0][0] = true;

        while let Some((r, c, cost)) = queue.pop_front() {
            if (r, c) == end {
                return cost.to_string();
            }
            let ir = r as i32;
            let ic = c as i32;
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let r = ir + dr;
                let c = ic + dc;
                if r < 0 || c < dc {
                    continue;
                }
                let r = r as usize;
                let c = c as usize;
                if r >= map_size || c >= map_size {
                    continue;
                }
                if !map[r][c] {
                    continue;
                }
                if visited[r][c] {
                    continue;
                }
                visited[r][c] = true;
                queue.push_back((r, c, cost + 1))
            }
        }
        String::from("0")
    }

    fn solve_part_2(input: String) -> String {
        let lines = &mut input.lines();
        let map_size: usize = lines.next().unwrap().parse().unwrap();
        let mut map: Vec<Vec<bool>> = vec![vec![true; map_size]; map_size];
        let _bytes: usize = lines.next().unwrap().parse().unwrap();
        for (r, c) in lines.map(|line| {
            let mut words = line.split(",");
            let r: usize = words.next().unwrap().parse().unwrap();
            let c: usize = words.next().unwrap().parse().unwrap();
            (r, c)
        }) {
            if !simulate(&mut map, (r, c)) {
                return format!("{},{}", r, c);
            }
        }
        String::from("0")
    }
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day18::test_input();
        let ans = Day18::solve_part_1(input);
        assert_eq!(ans, "22");
    }

    #[test]
    fn test_part_2() {
        let input = Day18::test_input();
        let ans = Day18::solve_part_2(input);
        assert_eq!(ans, "6,1");
    }
}
