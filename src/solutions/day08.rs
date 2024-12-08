use std::collections::{HashMap, HashSet};

use super::Solution;

pub struct Day08;

fn gcd(a: i32, b: i32) -> i32 {
    if b < a {
        gcd(b, a)
    } else {
        let rem = b % a;
        if rem == 0 {
            a
        } else {
            gcd(rem, a)
        }
    }
}

fn num_antinodes(
    bound: (usize, usize),
    antennas: &[(usize, usize)],
    seen: &mut HashSet<(i32, i32)>,
) {
    let (mx, my) = bound;
    let mx = mx as i32;
    let my = my as i32;
    let l = antennas.len();
    for i in 0..l {
        for j in (i + 1)..l {
            let a = antennas[i];
            let b = antennas[j];
            let (x1, y1) = a;
            let (x2, y2) = b;
            let x1 = x1 as i32;
            let x2 = x2 as i32;
            let y1 = y1 as i32;
            let y2 = y2 as i32;
            let dx = x2 - x1;
            let dy = y2 - y1;
            let x = x1 - dx;
            let y = y1 - dy;
            if 0 <= x && x < mx && 0 <= y && y < my {
                seen.insert((x, y));
            }
            let x = x2 + dx;
            let y = y2 + dy;
            if 0 <= x && x < mx && 0 <= y && y < my {
                seen.insert((x, y));
            }
        }
    }
}

fn num_antinodes_2(
    bound: (usize, usize),
    antennas: &[(usize, usize)],
    seen: &mut HashSet<(i32, i32)>,
) {
    let (mx, my) = bound;
    let mx = mx as i32;
    let my = my as i32;
    let l = antennas.len();
    for i in 0..l {
        for j in (i + 1)..l {
            let a = antennas[i];
            let b = antennas[j];
            let (x1, y1) = a;
            let (x2, y2) = b;
            let x1 = x1 as i32;
            let x2 = x2 as i32;
            let y1 = y1 as i32;
            let y2 = y2 as i32;
            let dx = x2 - x1;
            let dy = y2 - y1;
            let d = gcd(dx.abs(), dy.abs());
            let dx = dx / d;
            let dy = dy / d;
            let mut x = x1;
            let mut y = y1;
            seen.insert((x1, y1));
            seen.insert((x2, y2));
            loop {
                x -= dx;
                y -= dy;
                if x < 0 || x >= mx || y < 0 || y >= my {
                    break;
                } else {
                    seen.insert((x, y));
                }
            }
            let mut x = x1;
            let mut y = y1;
            loop {
                x += dx;
                y += dy;
                if x == x2 && y == y2 {
                    continue;
                } else if x < 0 || x >= mx || y < 0 || y >= my {
                    break;
                } else {
                    seen.insert((x, y));
                }
            }
        }
    }
}

impl Solution for Day08 {
    fn test_input() -> String {
        String::from(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        )
    }

    fn solve_part_1(_input: String) -> String {
        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

        let map: Vec<Vec<char>> = _input.lines().map(|line| line.chars().collect()).collect();
        let bound = (map.len(), map[0].len());

        for (r, row) in map.iter().enumerate() {
            for (c, ch) in row.iter().enumerate() {
                match ch {
                    '.' => {}
                    ch => {
                        let v = antennas.entry(*ch).or_insert(vec![]);
                        v.push((r, c));
                    }
                }
            }
        }

        let mut seen: HashSet<(i32, i32)> = HashSet::new();

        for (_key, poses) in antennas.iter() {
            num_antinodes(bound, poses, &mut seen);
        }

        seen.len().to_string()
    }

    fn solve_part_2(_input: String) -> String {
        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

        let map: Vec<Vec<char>> = _input.lines().map(|line| line.chars().collect()).collect();
        let bound = (map.len(), map[0].len());

        for (r, row) in map.iter().enumerate() {
            for (c, ch) in row.iter().enumerate() {
                match ch {
                    '.' => {}
                    ch => {
                        let v = antennas.entry(*ch).or_insert(vec![]);
                        v.push((r, c));
                    }
                }
            }
        }

        let mut seen: HashSet<(i32, i32)> = HashSet::new();

        for (_key, poses) in antennas.iter() {
            num_antinodes_2(bound, poses, &mut seen);
        }

        seen.len().to_string()
    }
}

#[cfg(test)]
mod day08_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day08::test_input();
        let ans = Day08::solve_part_1(input);
        assert_eq!(ans, "14");
    }

    #[test]
    fn test_part_2() {
        let input = Day08::test_input();
        let ans = Day08::solve_part_2(input);
        assert_eq!(ans, "34");
    }
}
