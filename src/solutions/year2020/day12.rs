use crate::Solution;

#[repr(u8)]
#[derive(Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn right(&self) -> Self {
        use Dir::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn opposite(&self) -> Self {
        self.right().right()
    }

    fn left(&self) -> Self {
        self.opposite().right()
    }

    fn delta(&self) -> (i32, i32) {
        use Dir::*;
        match self {
            North => (1, 0),
            South => (-1, 0),
            East => (0, 1),
            West => (0, -1),
        }
    }
    fn from(char: char) -> Self {
        use Dir::*;
        match char {
            'N' => North,
            'W' => West,
            'E' => East,
            'S' => South,
            _ => unimplemented!(),
        }
    }
}

enum Cmd {
    Forward(i32),
    Dir(Dir, i32),
    Turn(bool, i32), // right if true
}

impl Cmd {
    fn from(line: &str) -> Self {
        let ch = line.chars().next().unwrap();
        let num = line[1..].parse().unwrap();
        match ch {
            'F' => Self::Forward(num),
            'L' => Self::Turn(false, num),
            'R' => Self::Turn(true, num),
            ch => {
                let dir = Dir::from(ch);
                Self::Dir(dir, num)
            }
        }
    }
}

struct Ship {
    facing: Dir,
    pos: (i32, i32),
    waypoint: (i32, i32),
}

impl Ship {
    fn new() -> Self {
        Self {
            facing: Dir::East,
            pos: (0, 0),
            waypoint: (1, 10),
        }
    }

    fn apply(&mut self, cmd: Cmd) {
        let dir: Dir;
        let dist: i32;
        match cmd {
            Cmd::Dir(d, n) => {
                dir = d;
                dist = n;
            }
            Cmd::Forward(n) => {
                dir = self.facing;
                dist = n;
            }
            Cmd::Turn(b, n) => {
                if n % 90 != 0 {
                    panic!("{} % 90 != 0", n)
                }
                let n = n / 90;
                if b {
                    for _ in 0..n {
                        self.facing = self.facing.right()
                    }
                } else {
                    for _ in 0..n {
                        self.facing = self.facing.left()
                    }
                }
                dir = Dir::North;
                dist = 0;
            }
        }
        let (dx, dy) = dir.delta();
        self.pos.0 += dx * dist;
        self.pos.1 += dy * dist;
    }

    fn apply_line(&mut self, line: &str) {
        let cmd = Cmd::from(line);
        self.apply(cmd);
    }

    fn right(&mut self) {
        let (x, y) = self.waypoint;
        self.waypoint = (-y, x);
    }

    fn left(&mut self) {
        let (x, y) = self.waypoint;
        self.waypoint = (y, -x);
    }

    fn apply_2(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::Dir(dir, dist) => {
                let (x, y) = dir.delta();
                self.waypoint.0 += x * dist;
                self.waypoint.1 += y * dist;
            }
            Cmd::Forward(dist) => {
                let (dx, dy) = self.waypoint;
                self.pos.0 += dx * dist;
                self.pos.1 += dy * dist;
            }
            Cmd::Turn(is_right, amount) => {
                let num = amount / 90;
                if is_right {
                    for _ in 0..num {
                        self.right();
                    }
                } else {
                    for _ in 0..num {
                        self.left();
                    }
                }
            }
        }
    }

    fn apply_line_2(&mut self, line: &str) {
        let cmd = Cmd::from(line);
        self.apply_2(cmd);
    }
}

pub struct Day12 {}

impl Solution for Day12 {
    fn test_input() -> String {
        String::from(
            "F10
N3
F7
R90
F11",
        )
    }
    fn solve_part_1(input: String) -> String {
        let mut ship = Ship::new();
        for line in input.lines() {
            ship.apply_line(line);
        }
        (ship.pos.0.abs() + ship.pos.1.abs()).to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut ship = Ship::new();
        for line in input.lines() {
            ship.apply_line_2(line);
        }
        (ship.pos.0.abs() + ship.pos.1.abs()).to_string()
    }
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day12::test_input();
        let ans = Day12::solve_part_1(input);
        assert_eq!(ans, "25")
    }

    #[test]
    fn test_part_2() {
        let input = Day12::test_input();
        let ans = Day12::solve_part_2(input);
        assert_eq!(ans, "286")
    }
}
