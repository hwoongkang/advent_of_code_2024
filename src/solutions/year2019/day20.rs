use std::collections::{BinaryHeap, HashMap, VecDeque};

use crate::Solution;

pub struct Day20 {}

#[derive(PartialEq, Eq)]
struct State {
    depth: usize,
    dist: usize,
    at: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.dist.partial_cmp(&self.dist) {
            Some(std::cmp::Ordering::Equal) => other.depth.partial_cmp(&self.depth),
            ord => ord,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.dist.cmp(&self.dist) {
            std::cmp::Ordering::Equal => other.depth.cmp(&self.depth),
            ord => ord,
        }
    }
}

#[derive(Clone, Default, Debug)]
struct Portal {
    name: String,
    id: usize,
    pair: usize,
    location: (usize, usize),
    exit: (usize, usize),
    deepens: bool,
}

enum Tile {
    Void,
    Empty,
    Wall,
    Portal(Portal),
}

fn print_map(map: &Vec<Vec<Tile>>) {
    for row in map.iter() {
        println!();
        for tile in row.iter() {
            print!("{}", tile);
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Tile::Void => "_",
            Tile::Empty => ".",
            Tile::Wall => "#",

            Tile::Portal(portal) => {
                if portal.deepens {
                    "▣"
                } else {
                    "▢"
                }
            }
        };
        write!(f, "{}", str)
    }
}

fn adj(r: usize, c: usize, mr: usize, mc: usize) -> Vec<(usize, usize)> {
    let mut ans = vec![];
    let ir = r as i32;
    let ic = c as i32;

    let imr = mr as i32;
    let imc = mc as i32;
    for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let ir = ir + dr;
        let ic = ic + dc;
        if ir < 0 || ic < 0 || ir >= imr || ic >= imc {
            continue;
        }
        let r = ir as usize;
        let c = ic as usize;
        ans.push((r, c));
    }
    ans
}

fn parse(input: String) -> (Vec<Vec<Tile>>, Vec<Portal>) {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mr = chars.len();
    let mc = chars[0].len();
    let imr = mr as i32;
    let imc = mc as i32;

    let mut map: HashMap<String, Vec<((usize, usize), (usize, usize), bool)>> = HashMap::new();
    for r in 0..mr {
        for c in 0..mc {
            let ir = r as i32;
            let ic = c as i32;
            if chars[r][c].is_ascii_alphabetic() {
                let portal = (r, c);
                let char0 = chars[r][c];
                for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let ir = ir + dr;
                    let ic = ic + dc;
                    if ir < 0 || ic < 0 || ir >= imr || ic >= imc {
                        continue;
                    }
                    let r = ir as usize;
                    let c = ic as usize;
                    if chars[r][c] == '.' {
                        let entry = (r, c);
                        let ir = ir - 2 * dr;
                        let ic = ic - 2 * dc;
                        if ir < 0 || ic < 0 || ir >= imr || ic >= imc {
                            continue;
                        }
                        let r = ir as usize;
                        let c = ic as usize;
                        let char1 = chars[r][c];
                        let ir = ir - dr;
                        let ic = ic - dc;
                        let deepens = if ir < 0 || ic < 0 || ir >= imr || ic >= imc {
                            false
                        } else {
                            true
                        };

                        let (char0, char1) = if dr == 1 || dc == 1 {
                            (char0, char1)
                        } else {
                            (char1, char0)
                        };

                        let key: String = [char0, char1].into_iter().collect();
                        println!("PORTAL: {}", key);
                        let v = map.entry(key).or_insert(vec![]);
                        v.push((portal, entry, deepens));
                    }
                }
            }
        }
    }
    let mut ans: Vec<Vec<Tile>> = chars
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|ch| match ch {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    ' ' => Tile::Void,
                    _ => Tile::Void,
                })
                .collect()
        })
        .collect();

    let mut start_portal: Portal = Portal::default();
    let mut end_portal: Portal = Portal::default();

    println!("{:?}", map);

    for (key, v) in map.iter() {
        if key == "AA" {
            start_portal = Portal {
                name: "AA".to_string(),
                id: 0,
                pair: 0,
                location: v[0].0,
                exit: v[0].1,
                deepens: false,
            };

            continue;
        } else if key == "ZZ" {
            end_portal = Portal {
                name: "ZZ".to_string(),
                id: 0,
                pair: 0,
                location: v[0].0,
                exit: v[0].1,
                deepens: false,
            };

            continue;
        }
        println!("{:?}", key);
        let (portal0, entry0, delta0) = v[0];
        let (portal1, entry1, delta1) = v[1];
        if key == "KM" {
            println!("{:?} {:?} {:?}", portal0, entry0, delta0);
            println!("{:?} {:?} {:?}", portal1, entry1, delta1);
        }
        ans[portal0.0][portal0.1] = Tile::Portal(Portal {
            name: key.clone(),
            id: 0,
            pair: usize::MAX,
            deepens: delta0,
            location: portal0,
            exit: entry1,
        });
        ans[portal1.0][portal1.1] = Tile::Portal(Portal {
            name: key.clone(),
            id: 0,
            pair: usize::MAX,
            deepens: delta1,
            location: portal1,
            exit: entry0,
        });
    }
    let mut portals = vec![];
    for (r, row) in ans.iter_mut().enumerate() {
        for (c, tile) in row.iter_mut().enumerate() {
            if let Tile::Portal(portal) = tile {
                portal.id = portals.len();
                if (r, c) != portal.location {
                    panic!("PARSE WRONG");
                }
                portals.push(portal.clone());
            }
        }
    }

    for i in 0..portals.len() - 1 {
        let (first_half, second_half) = portals.split_at_mut(i + 1);

        let lhs = &mut first_half[i];

        if lhs.pair != usize::MAX {
            continue;
        }
        for rhs in second_half.iter_mut() {
            if lhs.name == rhs.name {
                lhs.pair = rhs.id;
                rhs.pair = lhs.id;
                break;
            }
        }
    }

    start_portal.id = portals.len();
    start_portal.pair = portals.len();

    ans[start_portal.location.0][start_portal.location.1] = Tile::Portal(start_portal.clone());
    portals.push(start_portal);

    end_portal.id = portals.len();
    end_portal.pair = portals.len();
    ans[end_portal.location.0][end_portal.location.1] = Tile::Portal(end_portal.clone());
    portals.push(end_portal);

    (ans, portals)
}

fn dfs(from: (usize, usize), map: &Vec<Vec<Tile>>, num_portals: usize) -> Vec<usize> {
    let mr = map.len();
    let mc = map[0].len();

    let imr = mr as i32;
    let imc = mc as i32;

    let mut adj = vec![usize::MAX; num_portals];
    let (r, c) = from;
    let mut visited = vec![vec![false; mc]; mr];
    visited[r][c] = true;
    let mut queue = VecDeque::from([((r, c), 0)]);

    while let Some(((r, c), dist)) = queue.pop_front() {
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
            if visited[r][c] {
                continue;
            }
            visited[r][c] = true;
            match &map[r][c] {
                Tile::Void | Tile::Wall => {
                    continue;
                }
                Tile::Empty => {}
                Tile::Portal(p) => {
                    adj[p.id] = dist - 1;
                }
            }
            queue.push_back(((r, c), dist + 1))
        }
    }
    adj
}

fn make_nodes(portals: &[Portal], map: &Vec<Vec<Tile>>) -> Vec<Vec<usize>> {
    let mut adj = vec![];
    for p in portals.iter() {
        let row = dfs(p.location, &map, portals.len());
        if p.name == "KM" && p.deepens {
            println!("KM(INSIDE) : {:?}", row);
        }
        adj.push(row);
    }

    adj
}

impl Solution for Day20 {
    fn test_input() -> String {
        String::new()
    }
    fn solve_part_1(input: String) -> String {
        let (map, portals) = parse(input);
        let l = portals.len();
        let start = portals[l - 2].exit;
        let end = portals[l - 1].exit;
        let mr = map.len();
        let mc = map[0].len();
        let mut visited = vec![vec![false; mc]; mr];
        visited[start.0][start.1] = true;
        let mut queue = VecDeque::from([(start, 0)]);
        println!("starting from {:?}", start);

        while let Some((pos, steps)) = queue.pop_front() {
            if pos == end {
                return (steps).to_string();
            }
            for (mut r, mut c) in adj(pos.0, pos.1, mr, mc) {
                match &map[r][c] {
                    Tile::Void | Tile::Wall => {
                        continue;
                    }
                    Tile::Empty => {}
                    Tile::Portal(portal) => {
                        r = portal.exit.0;
                        c = portal.exit.1;
                    }
                }
                if visited[r][c] {
                    continue;
                }
                visited[r][c] = true;
                queue.push_back(((r, c), steps + 1));
            }
        }
        String::new()
    }
    fn solve_part_2(input: String) -> String {
        let (map, portals) = parse(input);
        print_map(&map);
        let l = portals.len();

        let mut adj = make_nodes(&portals, &map);

        for row in adj.iter_mut() {
            row[l - 2] = usize::MAX;
        }

        let mut dists: HashMap<(usize, usize), usize> = HashMap::new();

        let mut heap = BinaryHeap::from([State {
            depth: 0,
            dist: 0,
            at: l - 2,
        }]);

        dists.insert((l - 2, 0), 0);

        for (p, portal) in portals.iter().enumerate() {
            let pair = &portals[portal.pair];
            if p != portal.id {
                panic!("INDEX NOT MATCHED")
            }
            if pair.name != portal.name {
                panic!("NAME NOT MATCHED")
            }
            if pair.id != portal.pair || pair.pair != portal.id {
                panic!("PAIR NOT MATCHED")
            }
            println!(
                "{}({}) - {}({})",
                portal.name, portal.deepens, pair.name, pair.deepens
            )
        }

        for (r, row) in adj.iter().enumerate() {
            println!(
                "adj for portal #{}:{}, {}",
                r,
                portals[r].name,
                if portals[r].deepens {
                    "INSIDE"
                } else {
                    "OUTSIDE"
                }
            );
            for (c, dist) in row.iter().enumerate() {
                if *dist == usize::MAX {
                    continue;
                }
                let dest = &portals[c];
                println!(
                    "\t{} steps to portal {}({})",
                    dist,
                    dest.name,
                    if dest.deepens { "INSIDE" } else { "OUTSIDE" }
                )
            }
        }

        while let Some(state) = heap.pop() {
            let at = &portals[state.at];

            if at.name == "ZZ" {
                return state.dist.to_string();
            }
            for (dest, delta) in adj[state.at]
                .iter()
                .enumerate()
                .filter(|(_, dist)| **dist != usize::MAX)
            {
                let portal = &portals[dest];
                if portal.name == "ZZ" {
                    if state.depth == 0 {
                        heap.push(State {
                            at: dest,
                            depth: 0,
                            dist: state.dist + delta,
                        })
                    }
                    continue;
                }

                if state.depth == 0 && !portal.deepens {
                    continue;
                }

                let mut depth = state.depth;
                if portal.deepens {
                    depth += 1;
                } else {
                    depth -= 1;
                }
                let dist = state.dist + delta + 1;
                let at = portal.pair;
                if let Some(prev) = dists.get(&(at, depth)) {
                    if *prev <= dist {
                        continue;
                    }
                }
                dists.insert((at, depth), dist);
                heap.push(State {
                    at: portal.pair,
                    depth,
                    dist,
                })
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod day20_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = String::from(
            "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ",
        );
        let ans = Day20::solve_part_1(input);
        assert_eq!(ans, "23");
        let input = String::from(
            "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ",
        );
        let ans = Day20::solve_part_1(input);
        assert_eq!(ans, "58");
    }

    #[test]
    fn test_part_2() {
        let input = String::from(
            "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ",
        );
        let ans = Day20::solve_part_2(input);
        println!("Small example done");
        assert_eq!(ans, "26");
        let input = String::from(
            "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ",
        );
        let ans = Day20::solve_part_2(input);
        assert_eq!(ans, "396")
    }
}
