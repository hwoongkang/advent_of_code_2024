use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::Solution;

struct SpaceObject {
    name: String,
    parent: Option<Rc<RefCell<SpaceObject>>>,
}

impl SpaceObject {
    fn new(name: String) -> Self {
        Self { name, parent: None }
    }
    fn num_orbits(&self) -> i32 {
        if let Some(parent) = &self.parent {
            1 + parent.borrow().num_orbits()
        } else {
            0
        }
    }
}

pub struct Day06 {}

impl Solution for Day06 {
    fn test_input() -> String {
        String::from(
            "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L",
        )
    }
    fn solve_part_1(input: String) -> String {
        let mut object_map: HashMap<String, Rc<RefCell<SpaceObject>>> = HashMap::new();
        for line in input.lines() {
            let mut names = line.split(")");
            let parent_name = names.next().unwrap().to_string();
            let child_name = names.next().unwrap().to_string();
            let parent = object_map
                .entry(parent_name.clone())
                .or_insert(Rc::new(RefCell::new(SpaceObject::new(parent_name))));
            let parent = parent.clone();
            let child = object_map
                .entry(child_name.clone())
                .or_insert(Rc::new(RefCell::new(SpaceObject::new(child_name))));
            child.borrow_mut().parent = Some(parent);
        }
        object_map
            .values()
            .map(|obj| obj.borrow().num_orbits())
            .sum::<i32>()
            .to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut object_map: HashMap<String, Rc<RefCell<SpaceObject>>> = HashMap::new();
        for line in input.lines() {
            let mut names = line.split(")");
            let parent_name = names.next().unwrap().to_string();
            let child_name = names.next().unwrap().to_string();
            let parent = object_map
                .entry(parent_name.clone())
                .or_insert(Rc::new(RefCell::new(SpaceObject::new(parent_name))));
            let parent = parent.clone();
            let child = object_map
                .entry(child_name.clone())
                .or_insert(Rc::new(RefCell::new(SpaceObject::new(child_name))));
            child.borrow_mut().parent = Some(parent);
        }
        let santa = object_map.get("SAN").unwrap();
        let you = object_map.get("YOU").unwrap();
        let mut seen = HashMap::new();
        let mut head = you.clone();
        let mut depth = 0;
        while let Some(parent) = &head.clone().borrow().parent.clone() {
            depth += 1;
            seen.insert(parent.borrow().name.clone(), depth);
            head = parent.clone();
        }
        let mut head = santa.clone();
        let mut depth = 0;
        while let Some(parent) = &head.clone().borrow().parent.clone() {
            depth += 1;
            if let Some(d) = seen.get(&parent.borrow().name) {
                return (depth + d - 2).to_string();
            }
            head = parent.clone();
        }

        String::new()
    }
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day06::test_input();
        let ans = Day06::solve_part_1(input);
        assert_eq!(ans, "42")
    }

    #[test]
    fn test_part_2() {
        let mut input = Day06::test_input();
        input += "\nK)YOU\nI)SAN";
        let ans = Day06::solve_part_2(input);
        assert_eq!(ans, "4")
    }
}
