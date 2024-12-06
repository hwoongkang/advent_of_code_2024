use std::collections::HashSet;

use super::Solution;

pub struct Day06;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos(i32, i32);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

use Dir::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct State(Pos, Dir);

impl Dir {
    fn right(&self) -> Dir {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn dp(&self) -> Pos {
        match self {
            Up => Pos(-1, 0),
            Down => Pos(1, 0),
            Left => Pos(0, -1),
            Right => Pos(0, 1),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
struct System {
    guard: State,
    map: Vec<Vec<bool>>,
    trace: Vec<Vec<bool>>,
}

impl System {
    fn size(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }
    fn from(input: String) -> Self {
        let mut pos: Pos = Pos(0, 0);
        let dir = Up;
        let map: Vec<Vec<bool>> = input
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .map(|(c, ch)| match ch {
                        '.' => true,
                        '#' => false,
                        '^' => {
                            pos = Pos(r as i32, c as i32);
                            true
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        System {
            guard: State(pos, dir),
            trace: vec![vec![false; map[0].len()]; map.len()],
            map,
        }
    }
    fn tick(&mut self) -> Option<usize> {
        let dp = self.guard.1.dp();
        let pos = self.guard.0;
        self.trace[pos.0 as usize][pos.1 as usize] = true;
        let (r, c) = (pos.0 + dp.0, pos.1 + dp.1);
        let mr = self.map.len() as i32;
        let mc = self.map[0].len() as i32;
        if r < 0 || c < 0 || r >= mr || c >= mc {
            Some(
                self.trace
                    .iter()
                    .map(|row| row.iter().filter(|&&x| x).count())
                    .sum(),
            )
        } else if self.map[r as usize][c as usize] {
            self.guard.0 = Pos(r, c);
            None
        } else {
            self.guard.1 = self.guard.1.right();
            None
        }
    }

    fn check_for_loop(&mut self) -> bool {
        let mut seen: HashSet<State> = HashSet::new();
        loop {
            let existed = !seen.insert(self.guard);

            if existed {
                break true;
            }

            if let Some(_) = self.tick() {
                break false;
            }
        }
    }
}

impl Solution for Day06 {
    fn test_input() -> String {
        String::from(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        )
    }

    fn solve_part_1(_input: String) -> String {
        let mut sys = System::from(_input);
        loop {
            if let Some(clock) = sys.tick() {
                break clock;
            }
        }
        .to_string()
    }

    fn solve_part_2(_input: String) -> String {
        let sys = System::from(_input);
        let (mr, mc) = sys.size();
        let mut ans = 0;
        for r in 0..mr {
            for c in 0..mc {
                let ir = r as i32;
                let ic = c as i32;
                if ir == sys.guard.0 .0 && ic == sys.guard.0 .1 {
                    continue;
                } else if !sys.map[r][c] {
                    continue;
                } else {
                    let mut sys = sys.clone();
                    sys.map[r][c] = false;
                    if sys.check_for_loop() {
                        ans += 1;
                    }
                }
            }
        }
        ans.to_string()
    }
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day06::test_input();
        let ans = Day06::solve_part_1(input);
        assert_eq!(ans, "41");
    }

    #[test]
    fn test_part_2() {
        let input = Day06::test_input();
        let ans = Day06::solve_part_2(input);
        assert_eq!(ans, "6");
    }
}
