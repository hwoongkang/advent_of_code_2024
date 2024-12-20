use std::collections::VecDeque;

use super::Solution;

#[derive(Clone)]
struct Maze {
    map: Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Maze {
    fn from(input: String) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let map = input
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .map(|(c, ch)| match ch {
                        '#' => false,
                        '.' => true,
                        'S' => {
                            start = (r, c);
                            true
                        }
                        'E' => {
                            end = (r, c);
                            true
                        }
                        _ => unimplemented!(),
                    })
                    .collect()
            })
            .collect();
        Self { map, start, end }
    }

    fn size(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }

    fn standard(&self) -> usize {
        let mut queue = VecDeque::from([(self.start, 0)]);
        let (mr, mc) = self.size();
        let mut visited = vec![vec![false; mc]; mr];
        visited[self.start.0][self.start.1] = true;
        while let Some(((r, c), cost)) = queue.pop_front() {
            if (r, c) == self.end {
                return cost;
            }
            let ir = r as i32;
            let ic = c as i32;
            for (dr, dc) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let ir = ir + dr;
                let ic = ic + dc;
                let r = ir as usize;
                let c = ic as usize;
                if !self.map[r][c] {
                    continue;
                }
                if visited[r][c] {
                    continue;
                }
                visited[r][c] = true;
                queue.push_back(((r, c), cost + 1));
            }
        }

        0
    }
}

fn solve_with_threshold(input: String, threshold: usize) -> String {
    let maze = Maze::from(input);
    let standard_time = maze.standard();
    let (mr, mc) = maze.size();
    let mut ans = 0;
    for r in 1..mr - 1 {
        println!("{} / 140", r);
        for c in 1..mc - 1 {
            let mut maze = maze.clone();
            if !maze.map[r][c] {
                maze.map[r][c] = true;
                let time = maze.standard();
                let saved = standard_time - time;
                if saved >= threshold {
                    ans += 1;
                }
            }
        }
    }
    ans.to_string()
}
pub struct Day20;

impl Solution for Day20 {
    fn test_input() -> String {
        String::from(
            "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
        )
    }

    fn solve_part_1(_input: String) -> String {
        solve_with_threshold(_input, 100)
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod day20_tests {
    use super::*;

    #[test]
    fn test_standard() {
        let input = Day20::test_input();
        let maze = Maze::from(input);
        assert_eq!(maze.standard(), 84)
    }

    #[test]
    fn test_part_1() {
        let input = Day20::test_input();
        let ans = solve_with_threshold(input, 6);
        assert_eq!(ans, "16");
    }

    #[test]
    fn test_part_2() {
        let input = Day20::test_input();
        let ans = Day20::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
