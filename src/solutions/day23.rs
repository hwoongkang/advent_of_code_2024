use std::collections::HashSet;

use super::Solution;

const MAX_NODE: usize = 25 * 26 + 25;
type CONNECTED = [[bool; MAX_NODE]; MAX_NODE];

fn id_to_node(id: usize) -> String {
    let a = id / 26;
    let b = id % 26;
    let a = a as u8;
    let a = 'a' as u8 + a;
    let a = a as char;
    let b = b as u8;
    let b = 'a' as u8 + b;
    let b = b as char;
    format!("{}{}", a, b)
}

fn node_id(node: &str) -> usize {
    let mut chars = node.chars();
    let a = chars.next().unwrap();
    let b = chars.next().unwrap();
    let a = a as u8 - 'a' as u8;
    let b = b as u8 - 'a' as u8;
    (a as usize) * 26 + (b as usize)
}

fn starts_with_t(num: usize) -> bool {
    let t = 't' as u32 - 'a' as u32;
    let t = t as usize;
    let t = t * 26;
    t <= num && num < t + 26
}

fn parse(input: String) -> (Vec<usize>, CONNECTED) {
    let mut seen: [bool; MAX_NODE] = [false; MAX_NODE];
    let mut nodes = vec![];
    let mut connected = [[false; MAX_NODE]; MAX_NODE];
    for line in input.lines() {
        let mut words = line.split("-");
        let a = node_id(words.next().unwrap());
        let b = node_id(words.next().unwrap());
        if !seen[a] {
            seen[a] = true;
            nodes.push(a);
        }
        if !seen[b] {
            seen[b] = true;
            nodes.push(b);
        }

        connected[a][b] = true;
        connected[b][a] = true;
    }
    nodes.sort();
    (nodes, connected)
}

fn build_three(nodes: &[usize], connected: &CONNECTED) -> HashSet<Vec<usize>> {
    let l = nodes.len();
    let mut set = HashSet::new();
    for i in 0..l {
        for j in i + 1..l {
            for k in j + 1..l {
                let i = nodes[i];
                let j = nodes[j];
                let k = nodes[k];

                if connected[i][j] && connected[j][k] && connected[k][i] {
                    set.insert(vec![i, j, k]);
                }
            }
        }
    }
    set
}

fn build_for_more(
    nodes: &[usize],
    connected: &CONNECTED,
    prev: HashSet<Vec<usize>>,
) -> HashSet<Vec<usize>> {
    let l = nodes.len();
    let mut set = HashSet::new();
    for mut v in prev {
        for i in 0..l {
            let i = nodes[i];
            if v.contains(&i) {
                continue;
            }
            if v.iter().all(|&j| connected[i][j]) {
                v.push(i);
                v.sort();
                set.insert(v.clone());
            }
        }
    }
    set
}

pub struct Day23;

impl Solution for Day23 {
    fn test_input() -> String {
        String::from(
            "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn",
        )
    }

    fn solve_part_1(input: String) -> String {
        let (nodes, connected) = parse(input);
        let threes = build_three(&nodes, &connected);
        let ans = threes
            .into_iter()
            .map(|v| (v[0], v[1], v[2]))
            .filter(|&(i, j, k)| starts_with_t(i) || starts_with_t(j) || starts_with_t(k))
            .count();
        ans.to_string()
    }

    fn solve_part_2(input: String) -> String {
        let (nodes, connected) = parse(input);
        let mut prev = build_three(&nodes, &connected);
        let mut num = 3;
        while prev.len() != 1 {
            println!("{} {}", num, prev.len());
            num += 1;
            prev = build_for_more(&nodes, &connected, prev);
        }

        println!("{} {}", num, prev.len());
        prev.into_iter()
            .next()
            .unwrap()
            .into_iter()
            .map(id_to_node)
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[cfg(test)]
mod day23_tests {
    use super::*;

    #[test]
    fn test_id() {
        let id = "co";
        assert_eq!(id_to_node(node_id(id)), id);
    }

    #[test]
    fn test_part_1() {
        let input = Day23::test_input();
        let ans = Day23::solve_part_1(input);
        assert_eq!(ans, "7");
    }

    #[test]
    fn test_part_2() {
        let input = Day23::test_input();
        let ans = Day23::solve_part_2(input);
        assert_eq!(ans, "co,de,ka,ta");
    }
}
