use crate::Solution;

pub struct Day04;

#[derive(Debug)]
enum Direction {
    N,
    E,
    W,
    S,
    SW,
    SE,
    NE,
    NW,
}

use Direction::*;

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            N => (1, 0),
            S => (-1, 0),
            E => (0, 1),
            W => (0, -1),
            NW => (1, -1),
            NE => (1, 1),
            SW => (-1, -1),
            SE => (-1, 1),
        }
    }
    fn all() -> [Self; 8] {
        [N, S, E, W, SW, SE, NW, NE]
    }
    fn opposite(&self) -> Self {
        match self {
            N => S,
            S => N,
            E => W,
            W => E,
            NW => SE,
            SW => NE,
            SE => NW,
            NE => SW,
        }
    }
}

enum XMasConfig {
    MMSS,
    MSMS,
    SMSM,
    SSMM,
}

use XMasConfig::*;
impl XMasConfig {
    fn all() -> [Self; 4] {
        [MMSS, MSMS, SMSM, SSMM]
    }
    fn valid(&self, center: &Pos, chars: &Vec<Vec<char>>) -> bool {
        if chars[center.0][center.1] != 'A' {
            return false;
        }

        let dir1;
        let dir2;
        match self {
            MMSS => {
                dir1 = NW;
                dir2 = NE;
            }
            MSMS => {
                dir1 = NW;
                dir2 = SW;
            }
            SMSM => {
                dir1 = SE;
                dir2 = NE;
            }
            SSMM => {
                dir1 = SE;
                dir2 = SW;
            }
        }
        let bounds = Pos(chars.len(), chars[0].len());
        let m1 = center.apply_delta(&bounds, dir1.delta());
        let m2 = center.apply_delta(&bounds, dir2.delta());
        let s1 = center.apply_delta(&bounds, dir1.opposite().delta());
        let s2 = center.apply_delta(&bounds, dir2.opposite().delta());
        if m1.is_none() || m2.is_none() || s1.is_none() || s2.is_none() {
            false
        } else {
            let m1 = m1.unwrap();
            let m2 = m2.unwrap();
            let s1 = s1.unwrap();
            let s2 = s2.unwrap();
            chars[m1.0][m1.1] == 'M'
                && chars[m2.0][m2.1] == 'M'
                && chars[s1.0][s1.1] == 'S'
                && chars[s2.0][s2.1] == 'S'
        }
    }
}

struct Pos(usize, usize);

impl Pos {
    fn apply_delta(&self, bound: &Pos, delta: (i32, i32)) -> Option<Pos> {
        let mut r = self.0 as i32;
        let mut c = self.1 as i32;
        let (dr, dc) = delta;
        r += dr;
        c += dc;
        if r < 0 || c < 0 {
            None
        } else {
            let r = r as usize;
            let c = c as usize;
            if r >= bound.0 || c >= bound.1 {
                None
            } else {
                Some(Pos(r, c))
            }
        }
    }
}

fn find_xmas(chars: Vec<Vec<char>>) -> usize {
    let size = Pos(chars.len(), chars[0].len());
    let xmas = vec!['X', 'M', 'A', 'S'];
    let mut ans = 0;
    for r in 0..size.0 {
        for c in 0..size.1 {
            'dir: for dir in Direction::all() {
                let mut now = Some(Pos(r, c));
                for i in 0..4 {
                    let Some(Pos(r, c)) = now else {
                        continue 'dir;
                    };
                    let ch = chars[r][c];
                    if ch != xmas[i] {
                        continue 'dir;
                    }
                    now = Pos(r, c).apply_delta(&size, dir.delta())
                }

                ans += 1;
            }
        }
    }
    ans
}

impl Solution for Day04 {
    fn test_input() -> String {
        String::from(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        )
    }

    fn solve_part_1(_input: String) -> String {
        let chars: Vec<Vec<char>> = _input.lines().map(|line| line.chars().collect()).collect();
        find_xmas(chars).to_string()
    }

    fn solve_part_2(_input: String) -> String {
        let chars: Vec<Vec<char>> = _input.lines().map(|line| line.chars().collect()).collect();
        let (mr, mc) = (chars.len(), chars[0].len());
        let mut ans = 0;
        for r in 0..mr {
            for c in 0..mc {
                for config in XMasConfig::all() {
                    if config.valid(&Pos(r, c), &chars) {
                        ans += 1;
                    }
                }
            }
        }
        ans.to_string()
    }
}

#[cfg(test)]
mod day04_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day04::test_input();
        let ans = Day04::solve_part_1(input);
        assert_eq!(ans, "18");
    }

    #[test]
    fn test_part_2() {
        let input = Day04::test_input();
        let ans = Day04::solve_part_2(input);
        assert_eq!(ans, "9");
    }
}
