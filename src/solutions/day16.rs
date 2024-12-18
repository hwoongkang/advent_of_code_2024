use std::{cmp::Reverse, collections::BinaryHeap};

use super::Solution;

const TURN_COST: usize = 1000;
const MOVE_COST: usize = 1;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

use Dir::*;

impl Dir {
    fn usize(&self) -> usize {
        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Left => (0, -1),
            Up => (-1, 0),
            Right => (0, 1),
            Down => (1, 0),
        }
    }

    fn right(&self) -> Self {
        match self {
            Left => Up,
            Up => Right,
            Right => Down,
            Down => Left,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct State {
    pos: (usize, usize),
    dir: Dir,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl State {
    fn coord(&self) -> (usize, usize, usize) {
        (self.pos.0, self.pos.1, self.dir.usize())
    }
    fn next(&self) -> Vec<Self> {
        let mut ans = vec![];
        let ir = self.pos.0 as i32;
        let ic = self.pos.1 as i32;
        // go straight
        let (dr, dc) = self.dir.delta();
        let r = (ir + dr) as usize;
        let c = (ic + dc) as usize;
        ans.push(State {
            pos: (r, c),
            dir: self.dir,
            cost: self.cost + MOVE_COST,
        });

        // go right
        let (dr, dc) = self.dir.right().delta();
        let r = (ir + dr) as usize;
        let c = (ic + dc) as usize;
        ans.push(State {
            pos: (r, c),
            dir: self.dir.right(),
            cost: self.cost + MOVE_COST + TURN_COST,
        });

        // go left
        let (dr, dc) = self.dir.right().right().right().delta();
        let r = (ir + dr) as usize;
        let c = (ic + dc) as usize;
        ans.push(State {
            pos: (r, c),
            dir: self.dir.right().right().right(),
            cost: self.cost + MOVE_COST + TURN_COST,
        });
        ans
    }
}

struct Maze {
    map: Vec<Vec<bool>>,
}

impl Maze {
    fn from(input: String) -> (Self, (usize, usize), (usize, usize)) {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let map = input
            .lines()
            .enumerate()
            .map(|(r, row)| {
                row.chars()
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
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        (Self { map }, start, end)
    }

    fn size(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }

    fn solve(&self, start: (usize, usize), end: (usize, usize)) -> usize {
        let (mr, mc) = self.size();
        let mut dists = vec![vec![vec![usize::MAX; 4]; mc]; mr];
        let state = State {
            pos: (start.0, start.1),
            dir: Right,
            cost: 0,
        };
        let (r, c, d) = state.coord();
        let mut heap = BinaryHeap::from([state]);
        dists[r][c][d] = 0;
        while let Some(state) = heap.pop() {
            if state.pos == end {
                return state.cost;
            }

            for ns in state.next() {
                let (r, c, d) = ns.coord();
                if !self.map[r][c] {
                    continue;
                }
                if dists[r][c][d] > ns.cost {
                    dists[r][c][d] = ns.cost;
                    heap.push(ns);
                }
            }
        }
        0
    }
}

pub struct Day16;

impl Solution for Day16 {
    fn test_input() -> String {
        String::from(
            "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
        )
    }

    fn solve_part_1(input: String) -> String {
        let (maze, start, end) = Maze::from(input);
        maze.solve(start, end).to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day16::test_input();
        let ans = Day16::solve_part_1(input);
        assert_eq!(ans, "11048");
    }
    #[test]
    fn test_part_1_2() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            .to_string();
        let ans = Day16::solve_part_1(input);
        assert_eq!(ans, "7036");
    }

    #[test]
    fn test_part_2() {
        let input = Day16::test_input();
        let ans = Day16::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
