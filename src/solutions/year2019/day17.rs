use crate::Solution;

use super::computer::{self, Computer};

pub struct Day17 {}

fn alignment_parameter(input: &String) -> usize {
    let mut sum = 0;
    let chars: Vec<Vec<char>> = input
        .lines()
        .filter_map(|line| {
            if line.trim().len() == 0 {
                None
            } else {
                Some(line.trim())
            }
        })
        .map(|line| line.chars().collect())
        .collect();
    for r in 1..(chars.len() - 1) {
        for c in 1..(chars[0].len() - 1) {
            let points = [(r, c), (r - 1, c), (r + 1, c), (r, c + 1), (r, c - 1)];
            if points.into_iter().all(|(r, c)| chars[r][c] == '#') {
                sum += r * c;
            }
        }
    }
    sum
}

enum Dir {
    Up,
    Left,
    Down,
    Right,
}
impl Dir {
    fn delta(&self) -> (i32, i32) {
        match self {
            Self::Up => (-1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
            Self::Down => (1, 0),
        }
    }
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
}

fn analytical_solution(input: &String) -> String {
    let mut ans: Vec<String> = vec![];

    let mut pos = (0, 0);
    let mut dir = Dir::Up;

    let mut map: Vec<Vec<char>> = input
        .lines()
        .filter_map(|line| {
            if line.trim().len() == 0 {
                None
            } else {
                Some(line.trim())
            }
        })
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| {
                    if ch == '^' {
                        pos = (r, c);
                        return '#';
                    }
                    ch
                })
                .collect()
        })
        .collect();

    let mr = map.len();
    let mc = map[0].len();
    // preprocess
    for r in 1..(mr - 1) {
        for c in 1..(mc - 1) {
            let points = [(r, c), (r - 1, c), (r + 1, c), (r, c + 1), (r, c - 1)];
            if points.into_iter().all(|(r, c)| map[r][c] == '#') {
                map[r][c] = 'O'
            }
        }
    }

    fn try_add(
        pos: &(usize, usize),
        delta: (i32, i32),
        map: &Vec<Vec<char>>,
    ) -> Option<(usize, usize)> {
        let mr = map.len();
        let mc = map[0].len();
        let &(r, c) = pos;
        let (dr, dc) = delta;
        let mut ir = r as i32;
        let mut ic = c as i32;
        let imr = mr as i32;
        let imc = mc as i32;
        ir += dr;
        ic += dc;
        if ir < 0 || ir >= imr || ic < 0 || ic >= imc {
            None
        } else {
            Some((ir as usize, ic as usize))
        }
    }

    fn turn(pos: &(usize, usize), dir: &mut Dir, map: &Vec<Vec<char>>) -> Option<char> {
        let left_delta = dir.left().delta();
        if let Some(p) = try_add(pos, left_delta, &map) {
            if map[p.0][p.1] == '#' {
                *dir = dir.left();
                return Some('L');
            }
        }
        let right_delta = dir.right().delta();
        if let Some(p) = try_add(pos, right_delta, &map) {
            if map[p.0][p.1] == '#' {
                *dir = dir.right();
                return Some('R');
            }
        }
        None
    }

    fn move_forward(pos: &mut (usize, usize), dir: &Dir, map: &mut Vec<Vec<char>>) -> usize {
        let mut sum = 0;
        loop {
            if let Some(next) = try_add(pos, dir.delta(), &map) {
                let next_tile = map[next.0][next.1];
                if next_tile == '#' || next_tile == 'O' {
                    if map[pos.0][pos.1] == '#' {
                        map[pos.0][pos.1] = '.'
                    };
                    *pos = next;
                    sum += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        sum
    }

    loop {
        let Some(t) = turn(&pos, &mut dir, &map) else {
            break;
        };
        ans.push(t.to_string());
        ans.push(move_forward(&mut pos, &dir, &mut map).to_string());
    }

    ans.join(",")
}

impl Solution for Day17 {
    fn test_input() -> String {
        String::from(
            "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..",
        )
    }
    fn solve_part_1(input: String) -> String {
        let mut computer = Computer::from(&input.lines().next().unwrap());
        let mut input = String::new();
        loop {
            match computer.run() {
                computer::Result::NeedsInput => panic!("not likely"),
                computer::Result::Halted(_) => break,
                computer::Result::Output(n) => input.push(char::from_u32(n as u32).unwrap()),
            }
        }
        println!("input: \n{}", input);
        alignment_parameter(&input).to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut computer = Computer::from(&input.lines().next().unwrap());
        let mut output = String::new();
        loop {
            match computer.run() {
                computer::Result::NeedsInput => panic!("not likely"),
                computer::Result::Halted(_) => break,
                computer::Result::Output(n) => output.push(char::from_u32(n as u32).unwrap()),
            }
        }
        println!("input: \n{}", output);

        println!("analytical_solution: {:?}", analytical_solution(&output));
        // solution: L,8,R,10,L,10,R,10,L,8,L,8,L,10,L,8,R,10,L,10,L,4,L,6,L,8,L,8,R,10,L,8,L,8,L,10,L,4,L,6,L,8,L,8,L,8,R,10,L,10,L,4,L,6,L,8,L,8,R,10,L,8,L,8,L,10,L,4,L,6,L,8,L,8

        // B,A,B,C,A,C,B,C,A,C
        // A = R,10,L,8,L,8,L,10
        // B = L,8,R,10,L,10
        // C = L,4,L,6,L,8,L,8
        let right = 'R' as u32 as i64;

        let left = 'L' as u32 as i64;
        let a = 'A' as u32 as i64;
        let b = 'B' as u32 as i64;
        let c = 'C' as u32 as i64;
        let main_routine = vec![b, a, b, c, a, c, b, c, a, c];

        let routine_a = vec![right, 10, left, 8, left, 8, left, 10];
        let routine_b = vec![left, 8, right, 10, left, 10];
        let routine_c = vec![left, 4, left, 6, left, 8, left, 8];

        fn as_input(routine: &[i64]) -> Vec<i64> {
            let comma = 44;
            let new_line = 10;
            let mut input = vec![];
            for (i, n) in routine.iter().enumerate() {
                if *n < 10 {
                    input.push(48 + n);
                } else if *n == 10 {
                    input.push(49);
                    input.push(48);
                } else {
                    input.push(*n);
                }
                let separator = if i == routine.len() - 1 {
                    new_line
                } else {
                    comma
                };

                input.push(separator)
            }
            input
        }

        let mut computer = Computer::from(input.lines().next().unwrap());

        computer.tape[0] = 2;

        computer.input_seq.append(&mut as_input(&main_routine));
        computer.input_seq.append(&mut as_input(&routine_a));
        computer.input_seq.append(&mut as_input(&routine_b));
        computer.input_seq.append(&mut as_input(&routine_c));
        computer.input_seq.push('n' as u32 as i64);
        computer.input_seq.push(10);

        loop {
            match computer.run() {
                computer::Result::NeedsInput => panic!("maybe not"),
                computer::Result::Output(_) => {
                    // println!("OUTPUT: {}", i);
                }
                computer::Result::Halted(n) => break println!("HALTED: {}", n),
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod day17_tests {
    use super::*;

    #[test]
    fn test_ascii() {
        let c = char::from_u32(35).unwrap();
        assert_eq!(c, '#');
        let c = char::from_u32(46).unwrap();
        assert_eq!(c, '.');
        let c = char::from_u32(10).unwrap();
        assert_eq!(c, '\n');
    }

    #[test]
    fn test_analytic() {
        let input = String::from(
            "#######...#####
#.....#...#...#
#.....#...#...#
......#...#...#
......#...###.#
......#.....#.#
^########...#.#
......#.#...#.#
......#########
........#...#..
....#########..
....#...#......
....#...#......
....#...#......
....#####......",
        );
        let ans = String::from("R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2");
        assert_eq!(analytical_solution(&input), ans)
    }

    #[test]
    fn test_part_1() {
        let input = Day17::test_input();
        let ans = alignment_parameter(&input);
        assert_eq!(ans, 76)
    }

    #[test]
    fn test_part_2() {
        let input = Day17::test_input();
        let ans = Day17::solve_part_2(input);
        assert_eq!(ans, "0")
    }
}
