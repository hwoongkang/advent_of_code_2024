use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::Solution;

struct Chemical {
    name: String,
    bulk: usize,
    ingredients: Vec<(usize, Rc<RefCell<Chemical>>)>,
}

impl Chemical {
    fn produce(
        &self,
        n: usize,
        storage: &mut HashMap<String, usize>,
        produced: &mut HashMap<String, usize>,
    ) {
        let stock = storage.entry(self.name.clone()).or_insert(0);
        if *stock >= n {
            return;
        }
        let n = n - *stock;
        let set = (n + (self.bulk - 1)) / self.bulk;

        for (num, ingredient) in self.ingredients.iter() {
            ingredient.borrow().produce(num * set, storage, produced);
            let ingredient_stock = storage.get_mut(&ingredient.borrow().name).unwrap();

            *ingredient_stock -= num * set;
        }
        let stock = storage.entry(self.name.clone()).or_insert(0);
        let trace = produced.entry(self.name.clone()).or_insert(0);
        *stock += set * self.bulk;
        *trace += set * self.bulk;
    }
}

impl std::fmt::Debug for Chemical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} => {} {}",
            self.ingredients
                .iter()
                .map(|(num, chemical)| { format!("{} {}", num, chemical.borrow().name) })
                .collect::<Vec<_>>()
                .join(", "),
            self.bulk,
            self.name
        )
    }
}

fn parse(input: String) -> HashMap<String, Rc<RefCell<Chemical>>> {
    let mut map: HashMap<String, Rc<RefCell<Chemical>>> = HashMap::new();

    let parse_chemical = |word: &str| -> (usize, String) {
        let mut words = word.trim().split_ascii_whitespace();
        (
            words.next().unwrap().parse().unwrap(),
            words.next().unwrap().to_string(),
        )
    };

    for line in input.lines() {
        let mut eq = line.split("=>");
        let lhs = eq.next().unwrap().split(",").map(parse_chemical);
        let (bulk, name) = parse_chemical(eq.next().unwrap());
        let rhs = {
            let rhs = map
                .entry(name.clone())
                .or_insert(Rc::new(RefCell::new(Chemical {
                    name,
                    bulk,
                    ingredients: vec![],
                })))
                .clone();
            rhs.borrow_mut().bulk = bulk;
            rhs
        };
        for (bulk, name) in lhs {
            let ingredient = map
                .entry(name.clone())
                .or_insert(Rc::new(RefCell::new(Chemical {
                    name,
                    bulk: 1,
                    ingredients: vec![],
                })))
                .clone();

            rhs.borrow_mut().ingredients.push((bulk, ingredient));
        }
    }

    map
}

pub struct Day14 {}

impl Day14 {
    fn _test_inputs() -> Vec<String> {
        vec![
            String::from(
                "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
            ),
            String::from(
                "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
            ),
            String::from(
                "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
            ),
            String::from(
                "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF",
            ),
            String::from(
                "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
            ),
        ]
    }
}

impl Solution for Day14 {
    fn test_input() -> String {
        String::from(
            "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
        )
    }
    fn solve_part_1(input: String) -> String {
        let map = parse(input);

        let mut storage = HashMap::new();
        let mut produced = HashMap::new();
        let fuel = map.get("FUEL").unwrap();
        fuel.borrow().produce(1, &mut storage, &mut produced);
        let ore = produced.get("ORE").unwrap();
        ore.to_string()
    }
    fn solve_part_2(input: String) -> String {
        let map = parse(input);

        let fuel = map.get("FUEL").unwrap();

        let trillion: usize = 1_000_000_000_000;

        let mut start = 0;
        let mut end = 1 << 40;

        while start < end {
            let mid = (start + end) / 2;

            let mut storage = HashMap::new();
            let mut produced = HashMap::new();

            fuel.borrow().produce(mid, &mut storage, &mut produced);
            let trace = produced.get("ORE").unwrap();
            if trace > &trillion {
                end = mid;
            } else {
                start = mid + 1;
            }
        }
        (end - 1).to_string()
    }
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    #[test]
    fn larger_part_1() {
        let inputs = Day14::_test_inputs();
        let answers = vec![31, 165, 13312, 180697, 2210736];
        for i in 0..inputs.len() {
            let ans = Day14::solve_part_1(inputs[i].clone());
            assert_eq!(ans, answers[i].to_string())
        }
    }
    #[test]
    fn test_part_1() {
        let input = Day14::test_input();
        let ans = Day14::solve_part_1(input);
        assert_eq!(ans, "31")
    }

    #[test]
    fn test_part_2() {
        let inputs = Day14::_test_inputs();
        let answers = vec![82892753, 5586022, 460664];
        for (i, input) in inputs[2..].into_iter().enumerate() {
            let ans = Day14::solve_part_2(input.clone());
            assert_eq!(ans, answers[i].to_string())
        }
    }
}
