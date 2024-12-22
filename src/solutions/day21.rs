use std::{
    collections::{BinaryHeap, HashMap, VecDeque},
    hash::Hash,
};

use super::Solution;

const NUMERIC_C: usize = 3;
const NUMERIC_R: usize = 4;
const DIRECIONAL_C: usize = 3;
const DIRECIONAL_R: usize = 2;

const DANGER: char = '_';

const NUMERIC_KEYPAD: [[char; NUMERIC_C]; NUMERIC_R] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [DANGER, '0', 'A'],
];

const DIRECTIONAL_KEYPAD: [[char; DIRECIONAL_C]; DIRECIONAL_R] =
    [[DANGER, '^', 'A'], ['<', 'v', '>']];

enum Cmd {
    Up,
    Down,
    Left,
    Right,
    Press,
}

use Cmd::*;

impl Cmd {
    fn from(ch: char) -> Self {
        match ch {
            '^' => Up,
            'v' => Down,
            '<' => Left,
            '>' => Right,
            'A' => Press,
            _ => unreachable!(),
        }
    }

    fn to(&self) -> char {
        match self {
            Up => '^',
            Down => 'v',
            Left => '<',
            Right => '>',
            Press => 'A',
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
            Press => unimplemented!(),
        }
    }

    fn all() -> [Self; 5] {
        [Up, Down, Left, Right, Press]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DirectionalKeypad(usize, usize);

#[derive(Clone, Debug, PartialEq, Eq)]
struct NumericKeypad(usize, usize);

trait Keypad: Sized {
    fn from(r: i32, c: i32) -> Self;
    fn get(&self) -> (i32, i32);
    fn to_string(&self) -> String {
        let (r, c) = self.get();
        format!("{}{}", r, c)
    }
    fn next(&self) -> Vec<Self> {
        vec![]
    }

    fn query(&self, r: i32, c: i32) -> Option<char>;

    // None if invalid
    // Some(None) if no output
    // Some(Some(char)) if output
    fn apply(&mut self, cmd: Cmd) -> Option<Option<char>> {
        let (ir, ic) = self.get();
        match cmd {
            Press => Some(self.query(ir, ic)),
            dir => {
                let (dr, dc) = dir.delta();
                let ir = ir + dr;
                let ic = ic + dc;
                if self.query(ir, ic).is_some() {
                    *self = Self::from(ir, ic);
                    Some(None)
                } else {
                    None
                }
            }
        }
    }
}

impl Keypad for DirectionalKeypad {
    fn from(r: i32, c: i32) -> Self {
        Self(r as usize, c as usize)
    }
    fn get(&self) -> (i32, i32) {
        (self.0 as i32, self.1 as i32)
    }
    fn query(&self, r: i32, c: i32) -> Option<char> {
        if r < 0 || c < 0 {
            None
        } else {
            let r = r as usize;
            let c = c as usize;
            if r >= DIRECIONAL_R || c >= DIRECIONAL_C {
                None
            } else {
                let ch = DIRECTIONAL_KEYPAD[r][c];
                if ch == DANGER {
                    None
                } else {
                    Some(ch)
                }
            }
        }
    }
    fn next(&self) -> Vec<Self> {
        let mut ans = vec![];
        let ir = self.0 as i32;
        let ic = self.1 as i32;
        for (dr, dc) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let ir = ir + dr;
            let ic = ic + dc;
            if self.query(ir, ic).is_some() {
                ans.push(Self(ir as usize, ic as usize));
            }
        }
        ans
    }
}

impl Keypad for NumericKeypad {
    fn from(r: i32, c: i32) -> Self {
        Self(r as usize, c as usize)
    }
    fn get(&self) -> (i32, i32) {
        (self.0 as i32, self.1 as i32)
    }
    fn query(&self, r: i32, c: i32) -> Option<char> {
        if r < 0 || c < 0 {
            None
        } else {
            let r = r as usize;
            let c = c as usize;
            if r >= NUMERIC_R || c >= NUMERIC_C {
                None
            } else {
                let ch = NUMERIC_KEYPAD[r][c];
                if ch == DANGER {
                    None
                } else {
                    Some(ch)
                }
            }
        }
    }
}

impl DirectionalKeypad {
    fn new() -> Self {
        Self(0, 2)
    }
}

impl NumericKeypad {
    fn new() -> Self {
        Self(3, 2)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Part1 {
    keypads: (NumericKeypad, [DirectionalKeypad; 2]),
    output: String,
    cost: usize,
}

impl Part1 {
    fn new() -> Self {
        Self {
            cost: 0,
            output: String::new(),
            keypads: (
                NumericKeypad::new(),
                [DirectionalKeypad::new(), DirectionalKeypad::new()],
            ),
        }
    }
}

impl Hash for Part1 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let str = format!(
            "{}{}{}{}",
            self.keypads.0.to_string(),
            self.keypads.1[0].to_string(),
            self.keypads.1[1].to_string(),
            self.output
        );
        str.hash(state);
    }
}

impl Ord for Part1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Part1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn dijkstra_part1(target: &str) -> usize {
    let mut costs: HashMap<Part1, usize> = HashMap::new();
    let initial_state = Part1::new();
    let mut heap: BinaryHeap<Part1> = BinaryHeap::from([initial_state]);

    fn insert(costs: &mut HashMap<Part1, usize>, heap: &mut BinaryHeap<Part1>, state: Part1) {
        let cost_now = state.cost;
        let entry = costs.entry(state.clone()).or_insert(usize::MAX);
        if &cost_now < entry {
            *entry = cost_now;
            heap.push(state);
        }
    }
    while let Some(state) = heap.pop() {
        if state.output == target {
            return state.cost;
        }
        for cmd in Cmd::all() {
            let mut state = state.clone();
            state.cost += 1;
            let valid = state.keypads.1[1].apply(cmd);
            let Some(output) = valid else {
                insert(&mut costs, &mut heap, state);
                continue;
            };
            let Some(cmd) = output else {
                insert(&mut costs, &mut heap, state);
                continue;
            };
            let cmd = Cmd::from(cmd);
            let valid = state.keypads.1[0].apply(cmd);
            let Some(output) = valid else {
                insert(&mut costs, &mut heap, state);
                continue;
            };
            let Some(cmd) = output else {
                insert(&mut costs, &mut heap, state);
                continue;
            };
            let cmd = Cmd::from(cmd);
            let valid = state.keypads.0.apply(cmd);
            let Some(output) = valid else {
                insert(&mut costs, &mut heap, state);
                continue;
            };
            let Some(num) = output else {
                insert(&mut costs, &mut heap, state);
                continue;
            };
            state.output.push(num);
            if target.starts_with(&state.output) {
                insert(&mut costs, &mut heap, state);
            }
        }
    }
    0
}

pub struct Day21;

impl Solution for Day21 {
    fn test_input() -> String {
        String::from(
            "029A
980A
179A
456A
379A",
        )
    }

    fn solve_part_1(input: String) -> String {
        let mut ans = 0usize;
        for line in input.lines() {
            let num: usize = (&line[..3]).parse().unwrap();
            let shortest = dijkstra_part1(line);

            ans += shortest * num;
        }
        ans.to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod day21_tests {
    use super::*;

    #[test]
    fn test_cmds() {
        let mut keypad = DirectionalKeypad::new();
        assert_eq!(keypad.get(), (0, 2));
        keypad.apply(Left);
        assert_eq!(keypad.get(), (0, 1));
        assert_eq!(keypad.apply(Press), Some(Some('^')));
        let output = keypad.apply(Left);
        assert!(output.is_none());
        assert_eq!(keypad.get(), (0, 1));

        let mut keypad = NumericKeypad::new();
        assert_eq!(keypad.get(), (3, 2));
        keypad.apply(Up);
        keypad.apply(Up);
        let output = keypad.apply(Press);
        assert_eq!(output, Some(Some('6')));
        assert!(keypad.apply(Up).is_some());
        assert!(keypad.apply(Up).is_none())
    }

    #[test]
    fn test_dijkstra() {
        let target = "029A";
        let ans = dijkstra_part1(target);
        assert_eq!(ans, 68)
    }

    #[test]
    fn test_part_1() {
        let input = Day21::test_input();
        let ans = Day21::solve_part_1(input);
        assert_eq!(ans, "126384");
    }

    #[test]
    fn test_part_2() {
        let input = Day21::test_input();
        let ans = Day21::solve_part_2(input);
        assert_eq!(ans, "");
    }
}

//v<A<AA>>^AvA<^A>Av<A<A>>^AvAA<^A>Av<<A>>^AAvA^Av<A>^AA<A>Av<A<A>>^AAAvA<^A>A
//<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

// 179A
//    <<  ^ A ^^ A >> A  vvv  A
// <<vAA>^A>A<AA>AvAA^A<vAAA>^A
// <<vAA>A>^AAvA<^A>AvA^A<<vA>>^AAvA^A<vA>^AA<A>A<<vA>A>^AAAvA<^A>A

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

// ^ - <A   - v<<A>>^A
// > - vA   - v<A>^A
// v - v<A  - v<A<A>>^A
// < - v<<A - v<A<AA>>^A

// ^< - <Av<A   = v<<A>>^Av<A<A>>^A
// <^ - v<<A>^A = v<A<AA>>^AvA<^A>A
// ^> - <Av>A   =
// >^ - vA<^A   =
// v< - v<A<A   =
//    -
// <v - v<<A>A  =
// v> - v<A>A   =
// >v - vA<A    =
