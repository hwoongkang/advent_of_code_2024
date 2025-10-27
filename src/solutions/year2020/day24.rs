use std::collections::{HashMap, HashSet};

use crate::Solution;

#[derive(Debug, Eq, PartialEq)]
enum Face {
    White,
    Black,
}

impl Face {
    fn flip(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl Dir {
    fn dp(&self) -> (i32, i32) {
        match self {
            Dir::E => (1, 0),
            Dir::W => (-1, 0),
            Dir::NW => (0, 1),
            Dir::SE => (0, -1),
            Dir::NE => (1, 1),
            Dir::SW => (-1, -1),
        }
    }
}

fn parse_line(line: &str) -> Vec<Dir> {
    let mut ans = vec![];
    let mut it = line.chars();
    loop {
        let Some(char) = it.next() else { break ans };
        match char {
            'w' => ans.push(Dir::W),
            'e' => ans.push(Dir::E),
            's' => match it.next() {
                Some('e') => ans.push(Dir::SE),
                Some('w') => {
                    ans.push(Dir::SW);
                }
                _ => panic!("WRONG INPUT"),
            },
            'n' => match it.next() {
                Some('e') => ans.push(Dir::NE),
                Some('w') => ans.push(Dir::NW),
                _ => panic!("WRONG INPUT"),
            },
            _ => panic!("WRONG INPUT"),
        }
    }
}

pub struct Day24 {}

fn tick(map: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut count: HashMap<(i32, i32), usize> = HashMap::new();

    for key in map.iter() {
        for dir in [Dir::E, Dir::NE, Dir::NW, Dir::W, Dir::SW, Dir::SE] {
            let (dx, dy) = dir.dp();
            let x = key.0 + dx;
            let y = key.1 + dy;
            let entry = count.entry((x, y)).or_insert(0);
            *entry += 1;
        }
    }

    let mut new_map = HashSet::new();

    for (pos, count) in count.into_iter() {
        let is_black = map.get(&pos).is_some();
        if is_black {
            if count == 1 || count == 2 {
                new_map.insert(pos);
            }
        } else {
            if count == 2 {
                new_map.insert(pos);
            }
        }
    }

    new_map
}

impl Solution for Day24 {
    fn test_input() -> String {
        String::from(
            "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew",
        )
    }
    fn solve_part_1(input: String) -> String {
        let map = part_1(input);
        map.values()
            .filter(|f| f == &&Face::Black)
            .count()
            .to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut set: HashSet<(i32, i32)> = part_1(input).into_keys().collect();
        for _ in 0..100 {
            set = tick(set);
        }
        set.len().to_string()
    }
}

fn part_1(input: String) -> HashMap<(i32, i32), Face> {
    let mut map: HashMap<(i32, i32), Face> = HashMap::new();
    for line in input.lines() {
        let mut x = 0;
        let mut y = 0;
        for dir in parse_line(line) {
            let (dx, dy) = dir.dp();
            x += dx;
            y += dy;
        }
        let entry = map.entry((x, y)).or_insert(Face::White);

        *entry = entry.flip();
    }
    map.into_iter()
        .filter(|(_k, v)| v == &Face::Black)
        .collect()
}
#[cfg(test)]
mod day24_tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "sewe";
        assert_eq!(parse_line(input), vec![Dir::SE, Dir::W, Dir::E]);
        let input = "nwwswee";
        assert_eq!(
            parse_line(input),
            vec![Dir::NW, Dir::W, Dir::SW, Dir::E, Dir::E]
        );
    }

    #[test]
    fn test_coordinate() {
        let input = "nwwswee";
        let mut x = 0;
        let mut y = 0;
        for dir in parse_line(input) {
            let (dx, dy) = dir.dp();
            x += dx;
            y += dy;
        }
        assert_eq!(x, 0);
        assert_eq!(y, 0);
    }

    #[test]
    fn test_part_1() {
        let input = Day24::test_input();
        let ans = Day24::solve_part_1(input);
        assert_eq!(ans, "10")
    }

    #[test]
    fn test_part_2() {
        let input = Day24::test_input();
        let ans = Day24::solve_part_2(input);
        assert_eq!(ans, "2208")
    }
}
