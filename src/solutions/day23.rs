use super::Solution;

const MAX_NODE: usize = 25 * 26 + 25;

const T: usize = (('t' as u32 - 'a' as u32) as usize) * 26;

fn node_id(node: &str) -> usize {
    let mut chars = node.chars();
    let a = chars.next().unwrap();
    let b = chars.next().unwrap();
    let a = a as u32 - 'a' as u32;
    let b = b as u32 - 'a' as u32;
    (a as usize) * 26 + (b as usize)
}

fn starts_with_t(num: usize) -> bool {
    let t = 't' as u32 - 'a' as u32;
    let t = t as usize;
    let t = t * 26;
    t <= num && num < t + 26
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
        let mut is_node: [bool; MAX_NODE] = [false; MAX_NODE];
        let mut connected: [[bool; MAX_NODE]; MAX_NODE] = [[false; MAX_NODE]; MAX_NODE];
        for line in input.lines() {
            let mut nodes = line.split("-");
            let a = node_id(nodes.next().unwrap());
            let b = node_id(nodes.next().unwrap());
            is_node[a] = true;
            is_node[b] = true;
            connected[a][b] = true;
            connected[b][a] = true;
        }
        let mut ans = 0;
        for i in 0..MAX_NODE {
            for j in i + 1..MAX_NODE {
                for k in j + 1..MAX_NODE {
                    if !is_node[i] || !is_node[j] || !is_node[k] {
                        continue;
                    }
                    if !starts_with_t(i) && !starts_with_t(j) && !starts_with_t(k) {
                        continue;
                    }
                    if connected[i][j] && connected[j][k] && connected[k][i] {
                        ans += 1;
                    }
                }
            }
        }
        ans.to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod day23_tests {
    use super::*;

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
