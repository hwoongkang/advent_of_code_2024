use crate::Solution;

use super::computer::{self, Computer};

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn right(&self) -> Self {
        self.left().left().left()
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

struct Robot {
    dir: Dir,
    pos: (usize, usize),
    map: Vec<Vec<bool>>,
    painted: Vec<Vec<bool>>,
    brain: Computer,
}

impl Robot {
    fn from(input: String) -> Self {
        Self {
            dir: Dir::Up,
            pos: (0, 0),
            map: vec![vec![false]],
            painted: vec![vec![true]],
            brain: Computer::from(&input),
        }
    }

    fn turn(&mut self, right: bool) {
        if right {
            self.dir = self.dir.right();
        } else {
            self.dir = self.dir.left();
        }
    }

    fn move_forward(&mut self) {
        let (dr, dc) = self.dir.delta();
        let mut r = self.pos.0 as i32;
        let mut c = self.pos.1 as i32;
        r += dr;
        c += dc;

        let mc = self.map[0].len();
        let imr = self.map.len() as i32;
        let imc = self.map[0].len() as i32;
        if r < 0 {
            r += 1;
            self.painted.insert(0, vec![false; mc]);
            self.map.insert(0, vec![false; mc]);
        } else if r >= imr {
            self.painted.push(vec![false; mc]);
            self.map.push(vec![false; mc]);
        }
        if c < 0 {
            c += 1;
            for row in self.map.iter_mut() {
                row.insert(0, false);
            }
            for row in self.painted.iter_mut() {
                row.insert(0, false);
            }
        } else if c >= imc {
            for row in self.map.iter_mut() {
                row.push(false);
            }
            for row in self.painted.iter_mut() {
                row.push(false);
            }
        }
        self.pos = (r as usize, c as usize);
    }

    fn count_painted(&self) -> usize {
        self.painted
            .iter()
            .flat_map(|row| row.iter())
            .filter(|b| **b)
            .count()
    }
    fn tick(&mut self) -> Option<usize> {
        let is_white = self.map[self.pos.0][self.pos.1];
        let input = if is_white { 1 } else { 0 };
        self.brain.add_input(input);
        match self.brain.run() {
            computer::Result::NeedsInput => panic!("Not needed yet"),
            computer::Result::Output(output) => {
                self.map[self.pos.0][self.pos.1] = output == 1;
                self.painted[self.pos.0][self.pos.1] = true;
            }
            computer::Result::Halted(_) => return Some(self.count_painted()),
        }
        match self.brain.run() {
            computer::Result::NeedsInput => panic!("Not needed yet"),
            computer::Result::Output(output) => {
                self.turn(output == 1);
                self.move_forward();
            }
            computer::Result::Halted(_) => return Some(self.count_painted()),
        }
        None
    }
}

pub struct Day11 {}

impl Solution for Day11 {
    fn test_input() -> String {
        String::new()
    }
    fn solve_part_1(input: String) -> String {
        let mut robot = Robot::from(input);
        loop {
            if let Some(ans) = robot.tick() {
                break ans.to_string();
            }
        }
    }
    fn solve_part_2(input: String) -> String {
        let mut robot = Robot::from(input);
        robot.map = vec![vec![true]];
        loop {
            if let Some(_) = robot.tick() {
                break;
            }
        }
        "\n".to_string()
            + &robot
                .map
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|is_white| if *is_white { '#' } else { '.' })
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n")
    }
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day11::test_input();
        let ans = Day11::solve_part_1(input);
        assert_eq!(ans, "0")
    }

    #[test]
    fn test_part_2() {
        let input = Day11::test_input();
        let ans = Day11::solve_part_2(input);
        assert_eq!(ans, "0")
    }
}
