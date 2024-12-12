use super::Solution;

pub struct Day12;

struct Plot {
    plants: Vec<Vec<char>>,
}

const dp: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

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
        for (dr, dc) in dp.iter() {
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
        String::from("0")
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
    fn test_part_2() {
        let input = Day12::test_input();
        let ans = Day12::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
