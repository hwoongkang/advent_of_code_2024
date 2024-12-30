use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use crate::Solution;

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

    #[allow(dead_code)]
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

// https://www.reddit.com/r/adventofcode/comments/1hj2odw/comment/m3cu31p/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

fn construct_path(from: char, to: char) -> Vec<String> {
    if from == to {
        return vec!["A".to_string()];
    }

    let is_numeric = from.is_ascii_digit() || to.is_ascii_digit();

    let keypad: &[[char; 3]] = if is_numeric {
        &NUMERIC_KEYPAD
    } else {
        &DIRECTIONAL_KEYPAD
    };

    let mut f = (0, 0);
    let mut t = (0, 0);
    for (r, row) in keypad.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            let r = r as i32;
            let c = c as i32;
            if &from == ch {
                f = (r, c)
            }
            if &to == ch {
                t = (r, c)
            }
        }
    }

    let dr = t.0 - f.0;
    let dc = t.1 - f.1;
    let dr = dr.signum();
    let dc = dc.signum();

    let rchar = if dr < 0 { '^' } else { 'v' };
    let cchar = if dc < 0 { '<' } else { '>' };

    let mut ans = vec![];

    if dr != 0 {
        let mut r = f.0;
        let mut c = f.1;
        let mut trace = vec![];
        let mut local = String::new();
        while r != t.0 {
            r += dr;
            trace.push((r as usize, c as usize));
            local.push(rchar);
        }
        while c != t.1 {
            c += dc;

            trace.push((r as usize, c as usize));
            local.push(cchar);
        }

        if trace
            .into_iter()
            .filter(|&(r, c)| keypad[r][c] == DANGER)
            .count()
            == 0
        {
            local.push('A');
            ans.push(local);
        }
    }
    if dc != 0 {
        let mut r = f.0;
        let mut c = f.1;
        let mut trace = vec![];
        let mut local = String::new();
        while c != t.1 {
            c += dc;
            trace.push((r as usize, c as usize));
            local.push(cchar);
        }
        while r != t.0 {
            r += dr;
            trace.push((r as usize, c as usize));
            local.push(rchar);
        }
        if trace
            .into_iter()
            .filter(|&(r, c)| keypad[r][c] == DANGER)
            .count()
            == 0
        {
            local.push('A');
            ans.push(local);
        }
    }

    ans
}

fn paths() -> HashMap<(char, char), Vec<String>> {
    let mut cache = HashMap::new();
    let all_chars = "0123456789<^>vA";
    for ch in all_chars.chars() {
        cache.insert((ch, ch), vec!["A".to_string()]);
    }
    let nums = "0123456789A".chars().collect::<Vec<_>>();
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            let from = nums[i];
            let to = nums[j];
            cache.insert((from, to), construct_path(from, to));
            let (from, to) = (to, from);
            cache.insert((from, to), construct_path(from, to));
        }
    }
    let nums = "<^>v>A".chars().collect::<Vec<_>>();
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            let from = nums[i];
            let to = nums[j];
            cache.insert((from, to), construct_path(from, to));
            let (from, to) = (to, from);
            cache.insert((from, to), construct_path(from, to));
        }
    }
    cache
}

fn solve_with_depth(target: &str, depth: usize) -> usize {
    let mut cache: HashMap<(String, usize), usize> = HashMap::new();
    let paths = paths();

    fn inner(
        target: &str,
        depth: usize,
        cache: &mut HashMap<(String, usize), usize>,
        paths: &HashMap<(char, char), Vec<String>>,
    ) -> usize {
        if depth == 0 {
            return target.len();
        }
        let mut prev = 'A';
        let mut local = 0;
        let mut cmd = String::new();
        for char in target.chars() {
            let subpaths = paths.get(&(prev, char)).unwrap();
            prev = char;
            let mut min = usize::MAX;
            let mut local_cmd = String::new();
            for path in subpaths {
                if let Some(cost) = cache.get(&(path.clone(), depth - 1)) {
                    if *cost < min {
                        min = *cost;
                        local_cmd += path;
                    }
                    continue;
                }
                let cost = inner(path, depth - 1, cache, paths);
                cache.insert((path.clone(), depth - 1), cost);
                if cost < min {
                    min = cost;
                    local_cmd += path;
                }
            }
            cmd += &local_cmd;
            local += min;
        }

        local
    }

    inner(target, depth, &mut cache, &paths)
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

    fn solve_part_2(input: String) -> String {
        let mut ans = 0usize;
        for line in input.lines() {
            let num: usize = (&line[..3]).parse().unwrap();
            let shortest = solve_with_depth(line, 26);

            ans += shortest * num;
        }
        ans.to_string()
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
    fn test_path() {
        let from = 'A';
        let to = '2';
        let paths = construct_path(from, to);
        assert_eq!(paths, vec!["^<A", "<^A"]);

        let from = 'A';
        let to = '1';
        let paths = construct_path(from, to);
        assert_eq!(paths, vec!["^<<A"]);

        let from = 'A';
        let to = 'v';
        let paths = construct_path(from, to);
        assert_eq!(paths, vec!["v<A", "<vA"]);
        let from = 'A';
        let to = '<';
        let paths = construct_path(from, to);
        assert_eq!(paths, vec!["v<<A"]);
        let from = 'A';
        let to = '^';
        let paths = construct_path(from, to);
        assert_eq!(paths, vec!["<A"]);
    }

    #[test]
    fn test_cheat() {
        let input = "029A";
        assert_eq!(solve_with_depth(input, 1), 12);
        assert_eq!(solve_with_depth(input, 2), 28);
        assert_eq!(solve_with_depth(input, 3), 68);
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
