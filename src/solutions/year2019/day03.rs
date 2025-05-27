use crate::Solution;

pub struct Day03 {}

fn parse(input: &str) -> Vec<Line> {
    let mut lines = vec![];
    let mut pos = Pos(0, 0);
    for word in input.split(",") {
        let (line, p) = Line::from(pos, word);
        pos = p;
        lines.push(line);
    }
    lines
}

fn parse_2(input: &str) -> Vec<Line2> {
    let mut lines = vec![];
    let mut pos = Pos(0, 0);
    let mut acc_cost = 0;
    for word in input.split(",") {
        let (line, new_cost, p) = Line2::from(acc_cost, pos, word);
        pos = p;
        acc_cost = new_cost;
        lines.push(line);
    }
    lines
}

impl Day03 {
    fn _input_2() -> String {
        String::from(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
        )
    }

    fn _input_3() -> String {
        String::from(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        )
    }
}

impl Solution for Day03 {
    fn test_input() -> String {
        String::from(
            "R8,U5,L5,D3
U7,R6,D4,L4",
        )
    }

    fn solve_part_1(input: String) -> String {
        let input = &mut input.lines();
        let line_a = parse(input.next().unwrap());
        let line_b = parse(input.next().unwrap());

        let mut ans = i32::MAX;
        for a in line_a.iter() {
            for b in line_b.iter() {
                if let Some(p) = a.crosses(b) {
                    ans = ans.min(p.dist())
                }
            }
        }
        ans.to_string()
    }
    fn solve_part_2(input: String) -> String {
        let input = &mut input.lines();
        let line_a = parse_2(input.next().unwrap());
        let line_b = parse_2(input.next().unwrap());

        let mut ans = i32::MAX;
        for a in line_a.iter() {
            for b in line_b.iter() {
                if let Some((_, cost)) = a.crosses(b) {
                    ans = ans.min(cost)
                }
            }
        }
        ans.to_string()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Pos(i32, i32);

impl Pos {
    fn dist(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Line2 {
    Horz(i32, (i32, i32), i32),
    Vert(i32, i32, (i32, i32)),
}

impl Line2 {
    fn new(acc_cost: i32, from: Pos, dist: i32, dir: Dir) -> (Self, i32, Pos) {
        let new_cost = acc_cost + dist;
        match dir {
            Dir::Up => (
                Line2::Vert(acc_cost, from.0, (from.1, from.1 + dist)),
                new_cost,
                Pos(from.0, from.1 + dist),
            ),
            Dir::Down => (
                Line2::Vert(acc_cost, from.0, (from.1, from.1 - dist)),
                new_cost,
                Pos(from.0, from.1 - dist),
            ),
            Dir::Left => (
                Line2::Horz(acc_cost, (from.0, from.0 - dist), from.1),
                new_cost,
                Pos(from.0 - dist, from.1),
            ),
            Dir::Right => (
                Line2::Horz(acc_cost, (from.0, from.0 + dist), from.1),
                new_cost,
                Pos(from.0 + dist, from.1),
            ),
        }
    }

    fn from(acc_cost: i32, pos: Pos, word: &str) -> (Self, i32, Pos) {
        let dist: i32 = word[1..].parse().unwrap();
        let dir: Dir = match word.chars().next().unwrap() {
            'R' => Dir::Right,
            'L' => Dir::Left,
            'U' => Dir::Up,
            'D' => Dir::Down,
            _ => panic!(""),
        };
        Self::new(acc_cost, pos, dist, dir)
    }

    fn crosses(&self, rhs: &Self) -> Option<(Pos, i32)> {
        match (self, rhs) {
            (Line2::Horz(_, _, _), Line2::Horz(_, _, _))
            | (Line2::Vert(_, _, _), Line2::Vert(_, _, _)) => None,
            (Line2::Horz(_, _, _), Line2::Vert(_, _, _)) => rhs.crosses(self),
            (Line2::Vert(cost_a, x, (y0, y1)), Line2::Horz(cost_b, (x0, x1), y)) => {
                let y_min = y0.min(y1);
                let y_max = y0.max(y1);
                let x_min = x0.min(x1);
                let x_max = x0.max(x1);
                if !(y_min < y && y < y_max && x_min < x && x < x_max) {
                    None
                } else {
                    let cost_a = cost_a + (y - y0).abs();
                    let cost_b = cost_b + (x - x0).abs();
                    Some((Pos(*x, *y), cost_a + cost_b))
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Line {
    Horz((i32, i32), i32),
    Vert(i32, (i32, i32)),
}

impl Line {
    fn from(pos: Pos, word: &str) -> (Self, Pos) {
        let dist: i32 = word[1..].parse().unwrap();
        let dir: Dir = match word.chars().next().unwrap() {
            'R' => Dir::Right,
            'L' => Dir::Left,
            'U' => Dir::Up,
            'D' => Dir::Down,
            _ => panic!(""),
        };
        Self::new(pos, dist, dir)
    }
    fn new(from: Pos, dist: i32, dir: Dir) -> (Self, Pos) {
        match dir {
            Dir::Up => (
                Line::Vert(from.0, (from.1, from.1 + dist)),
                Pos(from.0, from.1 + dist),
            ),
            Dir::Down => (
                Line::Vert(from.0, (from.1 - dist, from.1)),
                Pos(from.0, from.1 - dist),
            ),
            Dir::Left => (
                Line::Horz((from.0 - dist, from.0), from.1),
                Pos(from.0 - dist, from.1),
            ),
            Dir::Right => (
                Line::Horz((from.0, from.0 + dist), from.1),
                Pos(from.0 + dist, from.1),
            ),
        }
    }

    fn crosses(&self, rhs: &Self) -> Option<Pos> {
        match (self, rhs) {
            (Line::Horz(_, _), Line::Horz(_, _)) | (Line::Vert(_, _), Line::Vert(_, _)) => None,
            (Line::Horz(_, _), Line::Vert(_, _)) => rhs.crosses(self),
            (Line::Vert(x, (y_min, y_max)), Line::Horz((x_min, x_max), y)) => {
                if y_min < y && y < y_max && x_min < x && x < x_max {
                    Some(Pos(*x, *y))
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod day03_tests {
    use super::*;

    #[test]
    fn test_crosses() {
        let lhs = Line::Vert(3, (2, 5));
        let rhs = Line::Horz((2, 6), 3);
        assert_eq!(lhs.crosses(&rhs), Some(Pos(3, 3)))
    }

    #[test]
    fn test_part_1() {
        let input = Day03::test_input();
        let ans = Day03::solve_part_1(input);
        assert_eq!(ans, "6");
        let input = Day03::_input_2();
        let ans = Day03::solve_part_1(input);
        assert_eq!(ans, "159");
        let input = Day03::_input_3();
        let ans = Day03::solve_part_1(input);
        assert_eq!(ans, "135")
    }

    #[test]
    fn test_part_2() {
        let input = Day03::test_input();
        let ans = Day03::solve_part_2(input);
        assert_eq!(ans, "30");
        let input = Day03::_input_2();
        let ans = Day03::solve_part_2(input);
        assert_eq!(ans, "610");
        let input = Day03::_input_3();
        let ans = Day03::solve_part_2(input);
        assert_eq!(ans, "410")
    }
}
