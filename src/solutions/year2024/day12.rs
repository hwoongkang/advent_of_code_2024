use crate::Solution;

pub struct Day12;

struct Plot {
    plants: Vec<Vec<char>>,
}

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

use Dir::*;

impl Dir {
    fn all() -> [Self; 4] {
        [Left, Right, Up, Down]
    }

    fn dp(&self) -> (i32, i32) {
        match self {
            Left => (0, -1),
            Right => (0, 1),
            Up => (-1, 0),
            Down => (1, 0),
        }
    }
}

impl Plot {
    fn from(input: String) -> Self {
        Self {
            plants: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn size(&self) -> (usize, usize) {
        (self.plants.len(), self.plants[0].len())
    }

    fn isize(&self) -> (i32, i32) {
        let (r, c) = self.size();
        (r as i32, c as i32)
    }

    fn next(&self, pos: (usize, usize)) -> (usize, Vec<(usize, usize)>) {
        let (r, c) = pos;
        let me = self.plants[r][c];
        let ir = r as i32;
        let ic = c as i32;
        let (imr, imc) = self.isize();
        let mut perimeter = 0;
        let mut next = vec![];
        for (dr, dc) in Dir::all().map(|dir| dir.dp()).iter() {
            let nr = ir + dr;
            let nc = ic + dc;
            if nr < 0 || nc < 0 || nr >= imr || nc >= imc {
                perimeter += 1;
            } else {
                let r = nr as usize;
                let c = nc as usize;
                let neighbor = self.plants[r][c];
                if neighbor != me {
                    perimeter += 1;
                } else {
                    next.push((r, c));
                }
            }
        }
        (perimeter, next)
    }

    fn search(&self, pos: (usize, usize), visited: &mut Vec<Vec<bool>>) -> usize {
        let mut area = 0;
        let mut perimeter = 0;

        let mut stack = vec![pos];
        visited[pos.0][pos.1] = true;
        while let Some(pos) = stack.pop() {
            area += 1;
            let (p, n) = self.next(pos);
            perimeter += p;
            for (r, c) in n {
                if !visited[r][c] {
                    visited[r][c] = true;
                    stack.push((r, c));
                }
            }
        }

        area * perimeter
    }

    fn dfs(
        &mut self,
        pos: (usize, usize),
        visited: &mut Vec<Vec<bool>>,
    ) -> (char, usize, [usize; 4]) {
        let (r, c) = pos;
        let me = self.plants[r][c];
        let (mr, mc) = self.size();
        let mut min_r = mr;
        let mut max_r = 0;
        let mut min_c = mc;
        let mut max_c = 0;
        let mut stack = vec![pos];
        visited[r][c] = true;
        let mut area = 0;
        while let Some(p) = stack.pop() {
            min_r = min_r.min(p.0);
            max_r = max_r.max(p.0);
            min_c = min_c.min(p.1);
            max_c = max_c.max(p.1);
            area += 1;
            let (_, n) = self.next(p);

            self.plants[p.0][p.1] = '.';
            for (r, c) in n {
                if !visited[r][c] {
                    visited[r][c] = true;
                    stack.push((r, c));
                }
            }
        }
        (me, area, [min_r, max_r, min_c, max_c])
    }

    fn has_fence(&self, pos: (usize, usize), dir: Dir) -> bool {
        let (r, c) = pos;
        let me = self.plants[r][c];
        let ir = r as i32;
        let ic = c as i32;
        let (dr, dc) = dir.dp();
        let ir = ir + dr;
        let ic = ic + dc;
        let (imr, imc) = self.isize();
        if ir < 0 || ic < 0 || ir >= imr || ic >= imc {
            true
        } else {
            let r = ir as usize;
            let c = ic as usize;
            me != self.plants[r][c]
        }
    }

    fn num_sides(&mut self, dfs_result: (char, usize, [usize; 4])) -> usize {
        let mut ans = 0;
        let (me, area, [min_r, max_r, min_c, max_c]) = dfs_result;

        // count vertical sides
        for c in min_c..=max_c {
            let mut left_prev = false;
            let mut right_prev = false;
            for r in min_r..=max_r {
                let now = &mut self.plants[r][c];
                if *now != me {
                    left_prev = false;
                    right_prev = false;
                    continue;
                }
                *now = now.to_ascii_lowercase();
                let left = self.has_fence((r, c), Left);
                let right = self.has_fence((r, c), Right);
                if left && left != left_prev {
                    ans += 1;
                }
                left_prev = left;
                if right && right != right_prev {
                    ans += 1;
                }
                right_prev = right;
            }
        }
        // count vertical sides
        for r in min_r..=max_r {
            let mut up = false;
            let mut down = false;
            for c in min_c..=max_c {
                let now = &mut self.plants[r][c];
                if me != *now {
                    up = false;
                    down = false;
                    continue;
                }
                *now = now.to_ascii_lowercase();
                let u = self.has_fence((r, c), Up);
                let d = self.has_fence((r, c), Down);
                if u && up != u {
                    ans += 1;
                }
                up = u;
                if d && down != d {
                    ans += 1;
                }
                down = d;
            }
        }

        area * ans
    }

    fn restore(&mut self, pos: (usize, usize), ch: char) {
        let (r, c) = pos;
        let (mr, mc) = self.size();
        let mut min_r = mr;
        let mut max_r = 0;
        let mut min_c = mc;
        let mut max_c = 0;
        let mut stack = vec![pos];
        let mut visited = vec![vec![false; mc]; mr];
        visited[r][c] = true;

        while let Some(p) = stack.pop() {
            min_r = min_r.min(p.0);
            max_r = max_r.max(p.0);
            min_c = min_c.min(p.1);
            max_c = max_c.max(p.1);

            let (_, n) = self.next(p);

            self.plants[p.0][p.1] = ch;
            for (r, c) in n {
                if !visited[r][c] {
                    visited[r][c] = true;
                    stack.push((r, c));
                }
            }
        }
    }
}

impl Solution for Day12 {
    fn test_input() -> String {
        String::from(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        )
    }

    fn solve_part_1(_input: String) -> String {
        let plot = Plot::from(_input);
        let (mr, mc) = plot.size();
        let mut visited = vec![vec![false; mc]; mr];
        let mut ans = 0;
        for r in 0..mr {
            for c in 0..mc {
                if visited[r][c] {
                    continue;
                } else {
                    ans += plot.search((r, c), &mut visited)
                }
            }
        }
        ans.to_string()
    }

    fn solve_part_2(_input: String) -> String {
        let mut plot = Plot::from(_input);
        let (mr, mc) = plot.size();
        let mut visited = vec![vec![false; mc]; mr];
        let mut ans = 0;
        for r in 0..mr {
            for c in 0..mr {
                if !visited[r][c] {
                    let mut dfs_result = plot.dfs((r, c), &mut visited);
                    let restore_char = dfs_result.0;
                    dfs_result.0 = '.';

                    ans += plot.num_sides(dfs_result);
                    plot.restore((r, c), restore_char);
                }
            }
        }
        ans.to_string()
    }
}

fn _smaller_test_input() -> String {
    "AAAA
BBCD
BBCC
EEEC"
        .to_string()
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn test_smaller() {
        let input = _smaller_test_input();
        let ans = Day12::solve_part_1(input);
        assert_eq!(ans, "140")
    }

    #[test]
    fn test_part_1() {
        let input = Day12::test_input();
        let ans = Day12::solve_part_1(input);
        assert_eq!(ans, "1930");
    }

    #[test]
    fn test_smaller_2() {
        let input = _smaller_test_input();
        let ans = Day12::solve_part_2(input);
        assert_eq!(ans, "80")
    }

    #[test]
    fn test_part_2() {
        let input = Day12::test_input();
        let ans = Day12::solve_part_2(input);

        assert_eq!(ans, "1206");
    }
}
