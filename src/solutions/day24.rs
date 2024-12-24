use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::Solution;

enum Gate {
    AND,
    OR,
    XOR,
}

impl Gate {
    fn output(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::AND => a & b,
            Self::OR => a | b,
            Self::XOR => a ^ b,
        }
    }

    fn from(word: &str) -> Self {
        match word {
            "AND" => Self::AND,
            "OR" => Self::OR,
            "XOR" => Self::XOR,
            _ => unreachable!("Wrong input"),
        }
    }
}

struct Node {
    value: Option<u64>,
    depends: Option<(Gate, Rc<RefCell<Node>>, Rc<RefCell<Node>>)>,
}

impl Node {
    fn new() -> Self {
        Self {
            value: None,
            depends: None,
        }
    }
    fn get(&mut self) -> u64 {
        if let Some(val) = self.value {
            return val;
        }
        let Some((gate, left, right)) = &self.depends else {
            unreachable!("Illegal node constructed")
        };

        let left = left.borrow_mut().get();
        let right = right.borrow_mut().get();
        let value = gate.output(left, right);
        self.value = Some(value);

        value
    }
}

pub struct Day24;

impl Solution for Day24 {
    fn test_input() -> String {
        String::from(
            "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj",
        )
    }

    fn solve_part_1(input: String) -> String {
        let mut lines = input.lines();
        let mut nodes: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
        for line in &mut lines {
            if line == "" {
                break;
            }
            // y04: 1
            let mut words = line.split(": ");
            let name = words.next().unwrap().to_string();
            let value = words.next().unwrap().parse().unwrap();
            let node = Node {
                value: Some(value),
                depends: None,
            };
            let node = Rc::new(RefCell::new(node));
            nodes.insert(name, node);
        }
        for line in lines {
            // hwm AND bqk -> z03
            let mut words = line.split_ascii_whitespace();
            let left = words.next().unwrap();
            let gate = Gate::from(words.next().unwrap());
            let right = words.next().unwrap();
            let name = words.nth(1).unwrap();

            let left_node = nodes
                .entry(left.to_string())
                .or_insert(Rc::new(RefCell::new(Node::new())))
                .clone();

            let right_node = nodes
                .entry(right.to_string())
                .or_insert(Rc::new(RefCell::new(Node::new())))
                .clone();

            let me = nodes
                .entry(name.to_string())
                .or_insert(Rc::new(RefCell::new(Node::new())));

            me.borrow_mut().value = None;
            me.borrow_mut().depends = Some((gate, left_node, right_node))
        }
        let mut ans = 0;
        for (name, node) in nodes.iter_mut() {
            if !name.starts_with("z") {
                continue;
            }
            let index: usize = (&name[1..]).parse().unwrap();
            ans += node.borrow_mut().get() << index;
        }
        ans.to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod day24_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day24::test_input();
        let ans = Day24::solve_part_1(input);
        assert_eq!(ans, "2024");
    }

    #[test]
    fn test_part_2() {
        let input = Day24::test_input();
        let ans = Day24::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
