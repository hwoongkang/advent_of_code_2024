use std::collections::HashSet;

use crate::Solution;

pub struct Day03 {}

struct Rect {
    id: usize,
    pos: (usize, usize),
    size: (usize, usize),
}

impl Rect {
    fn from(line: &str) -> Self {
        let mut words = line.split_ascii_whitespace();
        let id = words.next().unwrap()[1..].parse().unwrap();

        let mut pos = words.nth(1).unwrap().trim_end_matches(':').split(",");

        let x = pos.next().unwrap().parse().unwrap();
        let y = pos.next().unwrap().parse().unwrap();
        let pos = (x, y);

        let mut size = words.next().unwrap().split("x");

        let w = size.next().unwrap().parse().unwrap();
        let h = size.next().unwrap().parse().unwrap();
        let size = (w, h);

        Self { id, pos, size }
    }
}

impl Solution for Day03 {
    fn test_input() -> String {
        String::from(
            "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2",
        )
    }
    fn solve_part_1(input: String) -> String {
        let rects: Vec<Rect> = input.lines().map(Rect::from).collect();

        let mut xs = HashSet::new();
        let mut ys = HashSet::new();

        for rect in rects.iter() {
            let (x, y) = rect.pos;
            let (w, h) = rect.size;
            xs.insert(x);
            xs.insert(x + w);
            ys.insert(y);
            ys.insert(y + h);
        }

        let mut xs: Vec<_> = xs.into_iter().collect();
        let mut ys: Vec<_> = ys.into_iter().collect();

        xs.sort();
        ys.sort();

        let mut grid = vec![vec![0; ys.len() - 1]; xs.len() - 1];

        for rect in rects.iter() {
            let (x, y) = rect.pos;
            let (w, h) = rect.size;
            let x0 = xs.iter().position(|&n| n == x).unwrap();
            let x1 = xs.iter().position(|&n| n == x + w).unwrap();
            let y0 = ys.iter().position(|&n| n == y).unwrap();
            let y1 = ys.iter().position(|&n| n == y + h).unwrap();

            for x in x0..x1 {
                for y in y0..y1 {
                    grid[x][y] += 1;
                }
            }
        }

        let mut ans = 0;

        for i in 0..(xs.len() - 1) {
            for j in 0..(ys.len() - 1) {
                if grid[i][j] > 1 {
                    let w = xs[i + 1] - xs[i];
                    let h = ys[j + 1] - ys[j];
                    ans += w * h
                }
            }
        }

        ans.to_string()
    }
    fn solve_part_2(input: String) -> String {
        let rects: Vec<Rect> = input.lines().map(Rect::from).collect();

        let mut xs = HashSet::new();
        let mut ys = HashSet::new();

        for rect in rects.iter() {
            let (x, y) = rect.pos;
            let (w, h) = rect.size;
            xs.insert(x);
            xs.insert(x + w);
            ys.insert(y);
            ys.insert(y + h);
        }

        let mut xs: Vec<_> = xs.into_iter().collect();
        let mut ys: Vec<_> = ys.into_iter().collect();

        xs.sort();
        ys.sort();

        let mut grid = vec![vec![0; ys.len() - 1]; xs.len() - 1];

        for rect in rects.iter() {
            let (x, y) = rect.pos;
            let (w, h) = rect.size;
            let x0 = xs.iter().position(|&n| n == x).unwrap();
            let x1 = xs.iter().position(|&n| n == x + w).unwrap();
            let y0 = ys.iter().position(|&n| n == y).unwrap();
            let y1 = ys.iter().position(|&n| n == y + h).unwrap();

            for x in x0..x1 {
                for y in y0..y1 {
                    grid[x][y] += 1;
                }
            }
        }

        for rect in rects.iter() {
            let (x, y) = rect.pos;
            let (w, h) = rect.size;
            let x0 = xs.iter().position(|&n| n == x).unwrap();
            let x1 = xs.iter().position(|&n| n == x + w).unwrap();
            let y0 = ys.iter().position(|&n| n == y).unwrap();
            let y1 = ys.iter().position(|&n| n == y + h).unwrap();

            let mut flag = true;
            'outer: for x in x0..x1 {
                for y in y0..y1 {
                    if grid[x][y] != 1 {
                        flag = false;
                        break 'outer;
                    }
                }
            }
            if flag {
                return rect.id.to_string();
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod day03_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day03::test_input();
        let ans = Day03::solve_part_1(input);
        assert_eq!(ans, "4")
    }

    #[test]
    fn test_part_2() {
        let input = Day03::test_input();
        let ans = Day03::solve_part_2(input);
        assert_eq!(ans, "3")
    }
}
