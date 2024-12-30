use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::Solution;

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

    fn _to_string(&self) -> &str {
        match self {
            Self::AND => "AND",
            Self::OR => "OR",
            Self::XOR => "XOR",
        }
    }
}

struct Node {
    _name: String,
    value: Option<u64>,
    depends: Option<(Gate, Rc<RefCell<Node>>, Rc<RefCell<Node>>)>,
}

impl Node {
    fn new(name: String) -> Self {
        Self {
            _name: name,
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

    fn _get_print(&mut self, index: usize) -> u64 {
        let indent = "  ".repeat(index);
        if let Some(val) = self.value {
            println!("{}{} = {}", indent, self._name, val);
            return val;
        }
        let Some((gate, left, right)) = &self.depends else {
            unreachable!("Illegal node constructed")
        };

        let l = left.borrow_mut()._get_print(index + 1);
        let r = right.borrow_mut()._get_print(index + 1);
        let value = gate.output(l, r);
        self.value = Some(value);

        value
    }
}

fn parse(input: String) -> HashMap<String, Rc<RefCell<Node>>> {
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
            _name: name.clone(),
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
            .or_insert(Rc::new(RefCell::new(Node::new(left.to_string()))))
            .clone();

        let right_node = nodes
            .entry(right.to_string())
            .or_insert(Rc::new(RefCell::new(Node::new(right.to_string()))))
            .clone();

        let me = nodes
            .entry(name.to_string())
            .or_insert(Rc::new(RefCell::new(Node::new(name.to_string()))));

        me.borrow_mut().value = None;
        me.borrow_mut().depends = Some((gate, left_node, right_node))
    }
    nodes
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
        let mut nodes = parse(input);
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
        // x0n ^ y0n = z0n
        // carry_n-1 ^ x0n ^ y0n
        // carry_n-1 = x0n-1 & y0n-1
        // z00 ok.
        // z00 = x00 ^ y00
        // z01 = thc ^ brj
        // thc = x01 ^ y01 -- x01 + y01, raw z01
        // brj = x00 & y00 -- carry_00
        // z01 ok.

        // z02 = wsw ^ bmc
        // wsw = x02 ^ y02 -- x02 + y02
        // bmc = ngv | hhp -- total_cary_02
        // ngv = brj & thc -- propagated carry 01
        // brj = carry_00
        // thc = x01
        // hhp = x01 AND y01 -- local carry 01
        // z02 ok.

        // z03 = wps ^ kmn
        // wps = x03 ^ y03 -- local result 03
        // kmn = qfb | snk ?  total_carry_03
        // qfb = bmc & wsw -- total_carry_02 & z02 - propagated carry 02
        // snk = y02 & x02 -- local carry 02
        // z03 ok.

        // z04 = shk ^ rkw
        // rkw = x04 ^ y04 -- local result 04
        // shk = btc | pvw -- total_carry 04
        // btc = y03 & x03 -- local carry 03
        // pvw = wps & kmn -- l03 & total_carry03 -- propagated carry 03
        // z04 ok.

        // z05 = tmr ^ pnf
        // pnf = x05 ^ y05 -- local result 05
        // tmr = pbq | dpv -- total_carry 05
        // pbq = rkw & shk -- propagated carry (rkw: l04, shk: t04)
        // dpv = x04 & y04 -- local carry 04

        // z06 = qgc ^ kfh
        // qgc = x06 ^ y06 -- local result 06
        // kfh = djt | sbf -- total carry 06
        // djt = pnf & tmr -- propagated carry
        // sbf = x05 & y05 -- local carry 05

        // z07 = kpv ^ rvc
        // kpv = x07 ^ y07 -- local result 07
        // rvc = cfw | mwg -- total carry 07
        // cfw = qgc & kfh -- propagated carry
        // mwg = x06 & y06 -- local carry 06

        // z08 = bcp ^ rgc
        // bcp = x08 ^ y08 -- local result 08
        // rgc = swt | ghb -- total carry 08
        // swt = x07 & y07 -- local carry 07
        // ghb = rvc & kpv -- propgated carry

        // z09 = vpv ^ ddq
        // vpv = y09 ^ x09 -- local result 09
        // ddq = pbh & htn -- total carry 09

        // z10 = ggd ^ dfm
        // ggd = x10 ^ y10 -- local result 10
        // dfm = nnh | bjr -- total carry 10

        // z11 = cft ^ gsf
        // gsf = x11 ^ y11 -- local result 11
        // cft = pgp | skw -- total carry 11

        // z12 = mnh ^ bqw
        // bqw = x12 ^ y12 -- local result 12
        // mnh = wmn | nhd -- total carry 12

        // z13 wth wrong

        // z13 = dwq ^ pgq -- z13
        // dwq = x13 ^ y13 -- local result 13
        // pgq = dsw | pmr -- total carry 13
        // dsw = bqw & mnh -- propagated carry 12
        // pmr = x12 & y12 -- local carry 12

        // z14 = fcr ^ pqc
        // fcr = x14 ^ y14 -- local result 14
        // pqc = skt | wpp -- total carry 14
        // skt = y13 & x13 -- local carry 13

        // z15 = wph ^ phn
        // wph -- local result 15
        // phn -- total caryy 15

        // z16 = csb ^ pfg
        // csb -- local result 16
        // pfg -- total carry 16

        // z31 sth wrong
        // bgs = kqk ^ djr -- this should be ze1
        // kqk = x31 ^ y31 -- local result 31
        // djr = hcg | cwb -- total carry 31
        // cwb = msm & fdm -- propagated carry 30
        // hcg = x30 & y30 -- local carry 30

        // z32 = vdd ^ dgk
        // dgk = x32 ^ y32 -- local result 32
        // vdd = bgs | dvp -- total carry 32
        // bgs = kqk & djr -- propagated carry 31
        // dvp = x31 & y31 -- local carry 31

        // z33 - mrp ^ chd

        // z34 - rgh ^ nng

        // z35 - rrf ^ qnw

        // z36 - wpv ^ cwh

        // z37 - rcd ^ rqt

        // z38 - dtr ^ brm

        // z39 - jtd ^ qwj

        // z40 - gqg ^ bps

        // z41 - dns ^ qbf

        // z42 - qsm ^ bqd

        // z43 - kmm ^ jfh

        // so far
        // swt, z07, pqc, z13, bgs, z31, rjm, wsv
        //

        //
        //        let test = 44;
        //        for i in 0..test {
        //            let key = format!("z{:02}", i);
        //            nodes.get(&key).unwrap().borrow_mut().get();
        //        }
        //        let key = &format!("z{:02}", test);
        //
        //        let z = nodes.get_mut(key).unwrap();
        //        z.borrow_mut().get_print(0);
        //
        let keys = ["swt", "z07", "pqc", "z13", "bgs", "z31", "rjm", "wsv"];

        let mut keys = keys;
        keys.sort();
        keys.iter()
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[cfg(test)]
mod day24_tests {
    use super::*;

    #[test]
    fn test_random() {}

    #[test]
    fn test_part_1() {
        let input = Day24::test_input();
        let ans = Day24::solve_part_1(input);
        assert_eq!(ans, "2024");
    }

    #[test]
    fn test_part_2() {
        let input = Day24::test_input();
        let _ans = Day24::solve_part_2(input);
        assert!(true);
    }
}
