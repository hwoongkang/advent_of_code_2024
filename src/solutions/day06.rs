use super::Solution;

pub struct Day06;

#[derive(Clone, Copy)]
struct Pos(i32, i32);

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

use Dir::*;

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

struct System {
    guard: (Pos, Dir),
    map: Vec<Vec<bool>>,
    clock: usize,
    trace: Vec<Vec<bool>>,
}

impl System {
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
            guard: (pos, dir),
            trace: vec![vec![false; map[0].len()]; map.len()],
            map,
            clock: 0,
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
        String::from("0")
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
        assert_eq!(ans, "41");
    }
}
