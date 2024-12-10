use std::collections::VecDeque;

use super::Solution;

struct Map {
    heights: Vec<Vec<u8>>,
}

impl Map {
    fn from(input: String) -> Self {
        Self {
            heights: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|ch| ch.to_digit(10).unwrap() as u8)
                        .collect()
                })
                .collect(),
        }
    }

    fn size(&self) -> (usize, usize) {
        (self.heights.len(), self.heights[0].len())
    }

    fn adj(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut ans = vec![];
        let (r, c) = pos;
        let (mr, mc) = self.size();
        if r > 0 {
            ans.push((r - 1, c));
        }
        if c > 0 {
            ans.push((r, c - 1));
        }
        if r < mr - 1 {
            ans.push((r + 1, c));
        }
        if c < mc - 1 {
            ans.push((r, c + 1));
        }
        ans
    }

    fn score(&self, pos: (usize, usize)) -> usize {
        let (r, c) = pos;
        if self.heights[r][c] != 0 {
            return 0;
        }
        let (mr, mc) = self.size();
        let mut ans = 0;

        let mut visited = vec![vec![false; mc]; mr];
        visited[r][c] = true;
        let mut stack = vec![(r, c)];
        while let Some((r, c)) = stack.pop() {
            let height = self.heights[r][c];
            if height == 9 {
                ans += 1;
            } else {
                for (nr, nc) in self.adj((r, c)) {
                    let nh = self.heights[nr][nc];
                    if !visited[nr][nc] && nh == height + 1 {
                        visited[nr][nc] = true;
                        stack.push((nr, nc));
                    }
                }
            }
        }

        ans
    }

    fn score_bfs(&self, pos: (usize, usize)) -> usize {
        let (r, c) = pos;
        if self.heights[r][c] != 0 {
            return 0;
        }

        let (mr, mc) = self.size();

        let mut counts: Vec<Vec<Option<usize>>> = vec![vec![None; mc]; mr];
        counts[r][c] = Some(1);

        let mut queue = VecDeque::from([(r, c)]);
        let mut ans = 0;
        let mut visited = vec![vec![false; mc]; mr];

        while let Some((r, c)) = queue.pop_front() {
            if visited[r][c] {
                continue;
            }
            visited[r][c] = true;
            let height = self.heights[r][c];
            let count = counts[r][c].unwrap();
            if height == 9 {
                ans += counts[r][c].unwrap();
            } else {
                for (nr, nc) in self.adj((r, c)) {
                    let nh = self.heights[nr][nc];
                    if nh != height + 1 {
                        continue;
                    }
                    let prev_count = counts[nr][nc].get_or_insert(0);
                    *prev_count += count;
                    queue.push_back((nr, nc));
                }
            }
        }

        ans
    }
}

pub struct Day10;

impl Solution for Day10 {
    fn test_input() -> String {
        String::from(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        )
    }

    fn solve_part_1(_input: String) -> String {
        let map = Map::from(_input);
        let (mr, mc) = map.size();
        let mut ans = 0;
        for r in 0..mr {
            for c in 0..mc {
                ans += map.score((r, c));
            }
        }
        ans.to_string()
    }

    fn solve_part_2(_input: String) -> String {
        let map = Map::from(_input);
        let (mr, mc) = map.size();
        let mut ans = 0;
        for r in 0..mr {
            for c in 0..mc {
                ans += map.score_bfs((r, c));
            }
        }
        ans.to_string()
    }
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day10::test_input();
        let ans = Day10::solve_part_1(input);
        assert_eq!(ans, "36");
    }

    #[test]
    fn test_part_2() {
        let input = Day10::test_input();
        let ans = Day10::solve_part_2(input);
        assert_eq!(ans, "81");
    }
}
