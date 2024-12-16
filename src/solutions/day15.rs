use super::Solution;

pub struct Day15;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Tile {
    Wall,
    Box,
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
    fn print(&self) {
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
        String::from("0")
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
    fn test_part_2() {
        let input = Day15::test_input();
        let ans = Day15::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
