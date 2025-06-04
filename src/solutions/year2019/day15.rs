use std::collections::{HashMap, VecDeque};

use crate::Solution;

use super::computer::{self, Computer};

#[derive(PartialEq, Eq)]
enum Tile {
    Empty(usize),
    Wall,
    Oxygen,
}

impl Tile {
    fn to(&self) -> char {
        match self {
            Self::Empty(_) => '.',
            Self::Wall => '#',
            Self::Oxygen => 'O',
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to())
    }
}

enum Dir {
    North,
    East,
    South,
    West,
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::North => 'N',
                Self::West => 'W',
                Self::East => 'E',
                Self::South => 'S',
            }
        )
    }
}

impl Dir {
    fn turn_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }
    fn turn_right(&self) -> Self {
        self.turn_left().turn_left().turn_left()
    }

    fn move_forward(&self) -> ((i32, i32), i64) {
        match self {
            Self::North => ((-1, 0), 1),
            Self::South => ((1, 0), 2),
            Self::West => ((0, -1), 3),
            Self::East => ((0, 1), 4),
        }
    }
}

enum StatusCode {
    HitWall,
    MoveSuccess,
    FoundOxygen,
}

impl StatusCode {
    fn from(n: i64) -> Self {
        match n {
            0 => Self::HitWall,
            1 => Self::MoveSuccess,
            2 => Self::FoundOxygen,
            _ => panic!("Unknown status code"),
        }
    }
}

fn print(map: &HashMap<(i32, i32), Tile>) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    for &(x, y) in map.keys() {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    let mx = (max_x - min_x + 1) as usize;
    let my = (max_y - min_y + 1) as usize;
    let mut board = vec![vec![' '; my]; mx];
    for ((x, y), tile) in map.iter() {
        let x = x - min_x;
        let y = y - min_y;
        let x = x as usize;
        let y = y as usize;
        board[x][y] = tile.to();
    }
    for row in board.iter() {
        for col in row.iter() {
            print!("{}", col);
        }
        println!();
    }
}

pub struct Day15 {}

impl Solution for Day15 {
    fn test_input() -> String {
        String::new()
    }
    fn solve_part_1(input: String) -> String {
        type Map = HashMap<(i32, i32), Tile>;
        let mut map: Map = HashMap::new();

        let mut computer = Computer::from(&input);
        // computer.input_seq = vec![3, 3, 3];
        let mut x = 0;
        let mut y = 0;
        let mut dir = Dir::North;

        map.insert((0, 0), Tile::Empty(0));

        let mut should_check_left = true;
        let mut cost = 0;

        loop {
            match computer.run() {
                computer::Result::NeedsInput => {
                    println!("board: ");
                    print(&map);
                    println!();
                    if should_check_left {
                        dir = dir.turn_left();
                    }
                    let (_, input) = dir.move_forward();
                    computer.add_input(input);
                }
                computer::Result::Output(n) => match StatusCode::from(n) {
                    StatusCode::FoundOxygen => {
                        println!("FOUND OXYGEN");
                        let ((dx, dy), _) = dir.move_forward();

                        map.insert((x + dx, y + dy), Tile::Oxygen);
                        cost += 1;
                        break;
                    }
                    StatusCode::HitWall => {
                        should_check_left = false;
                        let ((dx, dy), _) = dir.move_forward();
                        map.insert((x + dx, y + dy), Tile::Wall);
                        dir = dir.turn_right();
                    }
                    StatusCode::MoveSuccess => {
                        should_check_left = true;
                        let ((dx, dy), _) = dir.move_forward();
                        x += dx;
                        y += dy;
                        if let Some(Tile::Empty(c)) = map.get(&(x, y)) {
                            cost = cost.min(*c);
                            map.insert((x, y), Tile::Empty(cost));
                        } else {
                            cost += 1;
                            map.insert((x, y), Tile::Empty(cost));
                        }
                    }
                },
                computer::Result::Halted(_) => {
                    break;
                }
            }
        }

        map.insert((0, 0), Tile::Oxygen);
        print(&map);

        cost.to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut map: HashMap<(i32, i32), Tile> = HashMap::new();
        let computer = Computer::from(&input);
        let tape = computer.tape.clone();

        type BfsState = ((i32, i32), usize, Computer);
        let mut queue: VecDeque<BfsState> = VecDeque::from([((0, 0), 0, computer)]);

        map.insert((0, 0), Tile::Empty(0));

        while let Some((pos, cost, computer)) = queue.pop_front() {
            let history = computer.input_seq.clone();
            for dir in [Dir::North, Dir::South, Dir::West, Dir::East] {
                let ((dx, dy), input) = dir.move_forward();
                let mut new_computer = Computer::with(tape.clone(), history.clone());

                loop {
                    match new_computer.run() {
                        computer::Result::NeedsInput => {
                            new_computer.add_input(input);
                            break;
                        }
                        computer::Result::Output(_) => {
                            // ignore
                        }
                        computer::Result::Halted(_) => panic!("Shouldn't be here"),
                    }
                }

                let tile = match new_computer.run() {
                    computer::Result::Output(n) => match StatusCode::from(n) {
                        StatusCode::FoundOxygen => Tile::Oxygen,
                        StatusCode::HitWall => Tile::Wall,
                        StatusCode::MoveSuccess => Tile::Empty(0),
                    },
                    _ => unreachable!("ITS A BUG"),
                };
                let new_pos = (pos.0 + dx, pos.1 + dy);
                if map.get(&new_pos).is_none() {
                    match tile {
                        Tile::Oxygen | Tile::Empty(_) => {
                            queue.push_back((new_pos, cost + 1, new_computer));
                        }
                        _ => {}
                    }
                    map.insert(new_pos, tile);
                }
            }
        }

        let ((x, y), _) = map
            .iter()
            .filter(|(_, v)| **v == Tile::Oxygen)
            .next()
            .unwrap();

        let x = *x;
        let y = *y;

        print(&map);
        let mut queue = VecDeque::from([(0, x, y)]);
        let mut ans = 0;
        while let Some((time, x, y)) = queue.pop_front() {
            ans = ans.max(time);
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let next = (x + dx, y + dy);
                if let Some(tile) = map.get_mut(&next) {
                    if let Tile::Empty(_) = tile {
                        *tile = Tile::Oxygen;
                        queue.push_back((time + 1, x + dx, y + dy));
                    }
                }
            }
        }
        println!();
        print(&map);

        ans.to_string()
    }
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day15::test_input();
        let ans = Day15::solve_part_1(input);
        assert_eq!(ans, "0")
    }

    #[test]
    fn test_part_2() {
        let input = Day15::test_input();
        let ans = Day15::solve_part_2(input);
        assert_eq!(ans, "0")
    }
}
