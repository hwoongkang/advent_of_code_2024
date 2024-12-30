use crate::Solution;

pub struct Day15;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Tile {
    Wall,
    Box,
    Empty,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Tile2 {
    Wall,
    BoxLeft,
    BoxRight,
    Empty,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

use Dir::*;
use Tile::*;

impl Dir {
    fn from(ch: char) -> Self {
        match ch {
            '<' => Left,
            '>' => Right,
            '^' => Up,
            'v' => Down,
            _ => unimplemented!(),
        }
    }
    fn delta(&self) -> (i32, i32) {
        match self {
            Left => (0, -1),
            Right => (0, 1),
            Up => (-1, 0),
            Down => (1, 0),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct System {
    robot: (usize, usize),
    map: Vec<Vec<Tile>>,
}

impl System {
    fn from(input: &str) -> Self {
        let mut robot: (usize, usize) = (0, 0);
        let map: Vec<Vec<Tile>> = input
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .map(|(c, char)| match char {
                        '#' => Wall,
                        '.' => Empty,
                        'O' => Box,
                        '@' => {
                            robot = (r, c);
                            Empty
                        }
                        _ => unimplemented!(),
                    })
                    .collect()
            })
            .collect();
        Self { robot, map }
    }

    fn size(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }
    fn isize(&self) -> (i32, i32) {
        let (r, c) = self.size();
        (r as i32, c as i32)
    }

    fn exec(&mut self, cmd: Dir) {
        let (robot_r, robot_c) = self.robot;
        let robot_ir = robot_r as i32;
        let robot_ic = robot_c as i32;
        let (imr, imc) = self.isize();
        let (dr, dc) = cmd.delta();

        let mut iter_r = robot_ir;
        let mut iter_c = robot_ic;

        loop {
            iter_r += dr;
            iter_c += dc;
            if iter_r < 0 || iter_c < 0 || iter_r >= imr || iter_c >= imc {
                break;
            }
            let tile = &self.map[iter_r as usize][iter_c as usize];

            match tile {
                Box => continue,
                Wall => return,
                Empty => {
                    while !(iter_r == robot_ir && iter_c == robot_ic) {
                        let now_r = iter_r as usize;
                        let now_c = iter_c as usize;
                        iter_r -= dr;
                        iter_c -= dc;
                        let prev_r = iter_r as usize;
                        let prev_c = iter_c as usize;
                        self.map[now_r][now_c] = self.map[prev_r][prev_c];
                    }
                    iter_r += dr;
                    iter_c += dc;
                    self.robot = (iter_r as usize, iter_c as usize);
                    break;
                }
            }
        }
    }

    fn score(&self) -> usize {
        let mut ans = 0;
        for (r, row) in self.map.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if *tile == Box {
                    ans += 100 * r + c;
                }
            }
        }
        ans
    }
    fn _print(&self) {
        for row in self.map.iter() {
            for tile in row.iter() {
                match tile {
                    Empty => print!("."),
                    Wall => print!("#"),
                    Box => print!("O"),
                }
            }
            println!();
        }
    }
}
#[derive(PartialEq, Eq, Debug)]
struct System2 {
    robot: (usize, usize),
    map: Vec<Vec<Tile2>>,
}

impl System2 {
    fn from(input: &str) -> Self {
        let mut robot: (usize, usize) = (0, 0);
        let map: Vec<Vec<Tile2>> = input
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .map(|(c, char)| match char {
                        '#' => [Tile2::Wall, Tile2::Wall],
                        '.' => [Tile2::Empty, Tile2::Empty],
                        'O' => [Tile2::BoxLeft, Tile2::BoxRight],
                        '@' => {
                            robot = (r, 2 * c);
                            [Tile2::Empty, Tile2::Empty]
                        }
                        _ => unimplemented!(),
                    })
                    .flatten()
                    .collect()
            })
            .collect();
        Self { robot, map }
    }

    fn size(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }

    fn _print(&self) {
        let (mr, mc) = self.size();
        let mut new_map: Vec<Vec<char>> = vec![vec!['.'; mc]; mr];
        for (r, row) in self.map.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                match tile {
                    Tile2::Empty => new_map[r][c] = '.',
                    Tile2::Wall => new_map[r][c] = '#',
                    Tile2::BoxLeft => new_map[r][c] = '[',
                    Tile2::BoxRight => new_map[r][c] = ']',
                }
            }
        }
        new_map[self.robot.0][self.robot.1] = '@';
        for row in new_map.iter() {
            for ch in row.iter() {
                print!("{}", ch)
            }
            println!();
        }
    }

    // Robot, Empty, BoxLeft, BoxRight, Wall
    // Robot is on empty
    // exec starts on robot position
    // scan for "next position": child.
    // should move the "farthest" box from robot first: postorder traversal
    // if dir is left or right: no difficulties
    // if dir is up or right: should scan for both sides
    // if one of them is blocked by a wall, none should move.
    // scan first, execute in post-order.
    fn can_push(&self, from: (usize, usize), to: Dir) -> bool {
        let (r, c) = from;
        let ir = r as i32;
        let ic = c as i32;
        let (dr, dc) = to.delta();
        let nr = (ir + dr) as usize;
        let nc = (ic + dc) as usize;
        let ahead = self.map[nr][nc];
        match ahead {
            Tile2::Empty => true,
            Tile2::Wall => false,
            Tile2::BoxLeft => match to {
                Up | Down => {
                    let box_left = (nr, nc);
                    let box_right = (nr, nc + 1);
                    self.can_push(box_left, to) && self.can_push(box_right, to)
                }
                dir => self.can_push((nr, nc), dir),
            },
            Tile2::BoxRight => match to {
                Up | Down => {
                    let box_right = (nr, nc);
                    let box_left = (nr, nc - 1);
                    self.can_push(box_left, to) && self.can_push(box_right, to)
                }
                dir => self.can_push((nr, nc), dir),
            },
        }
    }
    fn push(&mut self, from: (usize, usize), to: Dir) {
        let (r, c) = from;
        let ir = r as i32;
        let ic = c as i32;
        let (dr, dc) = to.delta();
        let nr = (ir + dr) as usize;
        let nc = (ic + dc) as usize;
        let me = self.map[r][c];

        let ahead = self.map[nr][nc];
        if ahead == Tile2::Empty {
            self.map[nr][nc] = me;
            return;
        }
        match to {
            Left | Right => {
                self.push((nr, nc), to);
                self.map[nr][nc] = me;
            }
            Up | Down => match ahead {
                Tile2::BoxLeft => {
                    self.push((nr, nc), to);
                    self.map[nr][nc] = me;
                    self.push((nr, nc + 1), to);
                    self.map[nr][nc + 1] = Tile2::Empty;
                }
                Tile2::BoxRight => {
                    self.push((nr, nc), to);
                    self.map[nr][nc] = me;
                    self.push((nr, nc - 1), to);
                    self.map[nr][nc - 1] = Tile2::Empty;
                }
                _ => unimplemented!(),
            },
        }
    }

    fn exec(&mut self, dir: Dir) {
        let from = self.robot;
        if !self.can_push(from, dir) {
            return;
        }
        let (r, c) = from;
        let ir = r as i32;
        let ic = c as i32;
        let (dr, dc) = dir.delta();
        let nr = (ir + dr) as usize;
        let nc = (ic + dc) as usize;

        self.push(from, dir);
        self.robot = (nr, nc);
    }

    fn score(&self) -> usize {
        let mut ans = 0;
        let (mr, mc) = self.size();
        for r in 0..mr {
            for c in 0..mc {
                if self.map[r][c] == Tile2::BoxLeft {
                    ans += 100 * r + c
                }
            }
        }
        ans
    }
}

impl Solution for Day15 {
    fn test_input() -> String {
        String::from(
            "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        )
    }

    fn solve_part_1(_input: String) -> String {
        let mut parts = _input.split("\n\n");
        let mut sys = System::from(parts.next().unwrap());

        let cmds: Vec<Dir> = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| line.chars().map(Dir::from))
            .flatten()
            .collect();

        for cmd in cmds {
            sys.exec(cmd);
        }

        sys.score().to_string()
    }

    fn solve_part_2(_input: String) -> String {
        let mut parts = _input.split("\n\n");
        let mut sys = System2::from(parts.next().unwrap());

        let cmds: Vec<Dir> = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| line.chars().map(Dir::from))
            .flatten()
            .collect();

        for cmd in cmds {
            sys.exec(cmd);
        }

        sys.score().to_string()
    }
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    #[test]
    fn test_cmd() {
        let mut prev = System::from(
            "########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );
        let curr = System::from(
            "########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );
        prev.exec(Right);
        assert_eq!(prev, curr);
    }
    #[test]
    fn test_cmd_2() {
        let mut prev = System::from(
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );
        let curr = System::from(
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );
        prev.exec(Left);
        assert_eq!(prev, curr);
    }
    #[test]
    fn test_cmd_3() {
        let mut prev = System::from(
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );
        let curr = System::from(
            "########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );
        prev.exec(Up);
        assert_eq!(prev, curr);
    }

    #[test]
    fn test_cmd_4() {
        let mut prev = System::from(
            "########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );
        let curr = System::from(
            "########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########",
        );
        prev.exec(Right);
        prev.exec(Right);
        assert_eq!(prev, curr);
    }
    #[test]
    fn test_cmd_5() {
        let mut prev = System::from(
            "########
#....OO#
##.....#
#.....O#
#.#.O@.#
#...O..#
#...O..#
########",
        );
        let curr = System::from(
            "########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########",
        );
        prev.exec(Left);

        assert_eq!(prev, curr);
    }

    #[test]
    fn test_score() {
        let sys = System::from(
            "##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########",
        );
        assert_eq!(sys.score(), 10092);
    }

    #[test]
    fn test_smaller() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
            .to_string();
        let ans = Day15::solve_part_1(input);

        assert_eq!(ans, "2028");
    }

    #[test]
    fn test_part_1() {
        let input = Day15::test_input();
        let ans = Day15::solve_part_1(input);
        assert_eq!(ans, "10092");
    }

    #[test]
    fn test_part_2_smaller() {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"
            .to_string();
        let ans = Day15::solve_part_2(input); // 105+207+306
        assert_eq!(ans, "618");
    }

    #[test]
    fn test_part_2() {
        let input = Day15::test_input();
        let ans = Day15::solve_part_2(input);
        assert_eq!(ans, "9021");
    }
}
