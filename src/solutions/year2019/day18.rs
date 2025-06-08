use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use crate::Solution;

pub struct Day18 {}

const MAX_DIST: usize = 1_000_000_000;

enum Tile {
    Door(usize),
    Key(usize),
    Empty,
    Wall,
}

fn parse(input: String) -> (Vec<Vec<Tile>>, (usize, usize), u32) {
    let mut start = (0, 0);
    let mut keys = 0;
    let map: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| match ch {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    '@' => {
                        start = (r, c);
                        Tile::Empty
                    }
                    c => {
                        if c.is_ascii_lowercase() {
                            let diff = c as u32 - 'a' as u32;
                            keys += 1;
                            Tile::Key(diff as usize)
                        } else if c.is_ascii_uppercase() {
                            let diff = c as u32 - 'A' as u32;
                            Tile::Door(diff as usize)
                        } else {
                            unreachable!("BUG")
                        }
                    }
                })
                .collect()
        })
        .collect();
    (map, start, keys)
}

fn dfs(from: (usize, usize), num_keys: usize, map: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let mr = map.len();
    let mc = map[0].len();

    let imr = mr as i32;
    let imc = mc as i32;
    let mut adj = vec![(MAX_DIST, 0); num_keys];
    let (r, c) = from;
    let mut visited = vec![vec![false; mc]; mr];
    visited[r][c] = true;
    let mut stack = vec![(r, c, 0, 0)];
    while let Some((r, c, steps, required_keys)) = stack.pop() {
        let ir = r as i32;
        let ic = c as i32;
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let ir = ir + dr;
            let ic = ic + dc;
            if ir < 0 || ic < 0 || ir >= imr || ic >= imc {
                continue;
            }
            let r = ir as usize;
            let c = ic as usize;
            let mut new_keys = required_keys;
            if visited[r][c] {
                continue;
            }
            visited[r][c] = true;
            match map[r][c] {
                Tile::Door(i) => {
                    new_keys |= 1 << i;
                }
                Tile::Empty => {}
                Tile::Wall => {
                    continue;
                }
                Tile::Key(j) => {
                    adj[j] = (steps + 1, required_keys);
                }
            }

            stack.push((r, c, steps + 1, new_keys));
        }
    }
    adj
}

fn parse_more(
    map: &Vec<Vec<Tile>>,
    start: (usize, usize),
    num_keys: u32,
) -> (Vec<(usize, usize)>, Vec<Vec<(usize, usize)>>) {
    let n = num_keys as usize;
    let mut key_positions = vec![(0, 0); n as usize];
    for (r, row) in map.iter().enumerate() {
        for (c, tile) in row.iter().enumerate() {
            match tile {
                Tile::Key(i) => {
                    key_positions[*i] = (r, c);
                }
                _ => {}
            }
        }
    }
    let mut adj = vec![];

    for &key in key_positions.iter() {
        let mut adj_row = dfs(key, n, &map);
        adj_row.push((MAX_DIST, 0));
        adj.push(adj_row);
    }

    let mut from_start = dfs(start, key_positions.len(), &map);
    from_start.push((MAX_DIST, 0));
    adj.push(from_start);

    // println!("key_positions: {:?}", key_positions);
    for (_r, row) in adj.iter().enumerate() {
        let _name = |r: usize| -> String {
            if r < n {
                format!("#{}", r)
            } else {
                "start".to_string()
            }
        };
        // println!("adj for {}", _name(_r));
        for (_c, (steps, _required_keys)) in row.iter().enumerate() {
            let _steps = if steps == &MAX_DIST {
                "-1".to_string()
            } else {
                format!("{}", steps)
            };
            // print!("  {}: ({}, {:0b})", _name(_c), _steps, _required_keys)
        }
        // println!()
    }

    (key_positions, adj)
}

#[derive(PartialEq, Eq)]
struct State1 {
    index: usize,
    keys: usize,
    steps: usize,
}

#[derive(PartialEq, Eq)]
struct State2 {
    robots: [usize; 4],
    keys: usize,
    steps: usize,
}

impl State1 {
    fn num_keys(&self) -> u32 {
        self.keys.count_ones()
    }
}
impl State2 {
    fn num_keys(&self) -> u32 {
        self.keys.count_ones()
    }
}

impl PartialOrd for State1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.steps.partial_cmp(&self.steps) {
            Some(std::cmp::Ordering::Equal) => self.num_keys().partial_cmp(&other.num_keys()),
            ord => ord,
        }
    }
}

impl PartialOrd for State2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.steps.partial_cmp(&self.steps) {
            Some(std::cmp::Ordering::Equal) => self.num_keys().partial_cmp(&other.num_keys()),
            ord => ord,
        }
    }
}

impl Ord for State1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.steps.cmp(&self.steps) {
            std::cmp::Ordering::Equal => self.num_keys().cmp(&other.num_keys()),
            ord => ord,
        }
    }
}

impl Ord for State2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.steps.cmp(&self.steps) {
            std::cmp::Ordering::Equal => self.num_keys().cmp(&other.num_keys()),
            ord => ord,
        }
    }
}

fn compressed_part_1(input: String) -> usize {
    let (map, start, num_keys) = parse(input);
    let (key_positions, adj) = parse_more(&map, start, num_keys);
    let n = key_positions.len();
    let mut dists: HashMap<(usize, usize), usize> = HashMap::new();
    dists.insert((n, 0), 0);
    let mut heap = BinaryHeap::from([State1 {
        index: n,
        keys: 0,
        steps: 0,
    }]);
    while let Some(state) = heap.pop() {
        if state.num_keys() == num_keys {
            return state.steps;
        }
        for (index, &(delta, required_keys)) in adj[state.index].iter().enumerate() {
            if delta == MAX_DIST {
                continue;
            }
            if ((!state.keys) & required_keys) != 0 {
                continue;
            }
            let steps = state.steps + delta;
            let keys = state.keys | 1 << index;
            let hash_key = (index, keys);
            if let Some(prev) = dists.get(&hash_key) {
                if *prev <= steps {
                    continue;
                }
            }
            dists.insert(hash_key, steps);
            heap.push(State1 { index, keys, steps })
        }
    }
    0
}

fn _solve_part_1_backup(input: String) -> String {
    let (map, start, num_keys) = parse(input);
    let mr = map.len();
    let mc = map[0].len();
    let imr = mr as i32;
    let imc = mc as i32;

    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
    visited.insert((start.0, start.1, 0));
    let mut queue = VecDeque::from([(start, 0usize, 0usize)]);
    while let Some((pos, keys, steps)) = queue.pop_front() {
        if keys.count_ones() == num_keys {
            return steps.to_string();
        }
        let (r, c) = pos;
        let ir = r as i32;
        let ic = c as i32;
        for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let ir = ir + dr;
            let ic = ic + dc;
            if ir < 0 || ir >= imr || ic < 0 || ic >= imc {
                continue;
            }
            let r = ir as usize;
            let c = ic as usize;
            let mut new_keys = keys;
            match map[r][c] {
                Tile::Wall => {
                    continue;
                }
                Tile::Empty => {}
                Tile::Key(i) => {
                    new_keys |= 1 << i;
                }
                Tile::Door(i) => {
                    if new_keys & (1 << i) == 0 {
                        continue;
                    }
                }
            }

            if visited.insert((r, c, new_keys)) {
                queue.push_back(((r, c), new_keys, steps + 1))
            }
        }
    }
    String::new()
}
impl Solution for Day18 {
    fn test_input() -> String {
        String::from(
            "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################",
        )
    }

    fn solve_part_1(input: String) -> String {
        // println!("Compressed");
        compressed_part_1(input).to_string()
    }

    fn solve_part_2(input: String) -> String {
        let (mut map, start, num_keys) = parse(input);
        let (r, c) = start;
        let starts = [
            (r - 1, c - 1),
            (r - 1, c + 1),
            (r + 1, c - 1),
            (r + 1, c + 1),
        ];
        for (r, c) in [(r, c), (r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
            map[r][c] = Tile::Wall;
        }
        let mut adjs = vec![];
        for i in 0..4 {
            let (_, adj) = parse_more(&map, starts[i], num_keys);
            adjs.push(adj);
            // println!()
        }
        let n = num_keys as usize;
        let mut dists: HashMap<([usize; 4], usize), usize> = HashMap::new();

        let mut heap = BinaryHeap::from([State2 {
            robots: [n; 4],
            keys: 0,
            steps: 0,
        }]);
        while let Some(state) = heap.pop() {
            if state.num_keys() == num_keys {
                return state.steps.to_string();
            }

            for robot_index in 0..4 {
                let robot_is_on = state.robots[robot_index];
                for (index, &(delta, required_keys)) in
                    adjs[robot_index][robot_is_on].iter().enumerate()
                {
                    if delta == MAX_DIST {
                        continue;
                    }
                    if ((!state.keys) & required_keys) != 0 {
                        continue;
                    }
                    let steps = state.steps + delta;
                    let keys = state.keys | 1 << index;
                    let mut new_robots = state.robots.clone();
                    new_robots[robot_index] = index;
                    let hash_key = (new_robots.clone(), keys);
                    if let Some(prev) = dists.get(&hash_key) {
                        if *prev <= steps {
                            continue;
                        }
                    }
                    dists.insert(hash_key, steps);
                    heap.push(State2 {
                        robots: new_robots,
                        keys,
                        steps,
                    })
                }
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = String::from(
            "#########
#b.A.@.a#
#########",
        );
        let ans = _solve_part_1_backup(input);
        assert_eq!(ans, "8");
        let input = Day18::test_input();
        let ans = _solve_part_1_backup(input);
        assert_eq!(ans, "86")
    }

    #[test]
    fn test_part_1_compressed() {
        let input = Day18::test_input();
        let ans = compressed_part_1(input);
        assert_eq!(ans, 86)
    }

    #[test]
    fn test_part_2() {
        let input = String::from(
            "#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######",
        );
        let ans = Day18::solve_part_2(input);
        assert_eq!(ans, "8");

        let input = String::from(
            "###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############",
        );
        let ans = Day18::solve_part_2(input);
        assert_eq!(ans, "24");

        let input = String::from(
            "#############
#DcBa.#.GhKl#
#.###...#I###
#e#d#.@.#j#k#
###C#...###J#
#fEbA.#.FgHi#
#############",
        );
        let ans = Day18::solve_part_2(input);
        assert_eq!(ans, "32");

        let input = String::from(
            "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############",
        );
        let ans = Day18::solve_part_2(input);
        assert_eq!(ans, "72");
    }
}
