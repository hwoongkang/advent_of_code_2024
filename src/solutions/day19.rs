use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::Solution;

pub struct Day19;

#[derive(Debug)]
struct Node {
    is_leaf: bool,
    children: HashMap<char, Rc<RefCell<Node>>>,
}

impl Node {
    fn new() -> Self {
        Self {
            is_leaf: false,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, word: &[char]) {
        if word.len() == 0 {
            self.is_leaf = true;
        } else {
            let key = word[0];
            let remaining = &word[1..];
            let entry = self
                .children
                .entry(key)
                .or_insert(Rc::new(RefCell::new(Self::new())));
            entry.borrow_mut().insert(remaining);
        }
    }

    fn print(&self, indent: usize) {
        let tab = "  ".repeat(indent);
        println!("{}is_leaf: {}", tab, self.is_leaf);
        for (key, child) in self.children.iter() {
            println!("{}{}", tab, key);
            child.borrow().print(indent + 1);
        }
    }

    fn construct(&self, word: &str, root: &Self) -> bool {
        println!("{}", word);
        if word == "" {
            return false;
        }
        let key = word.chars().next().unwrap();
        let remaining = &word[1..];

        let child = self.children.get(&key);

        if remaining == "" {
            if let Some(child) = child {
                child.borrow().is_leaf
            } else {
                false
            }
        } else {
            if let Some(child) = child {
                println!("{} {} {}", key, child.borrow().is_leaf, remaining);
                if child.borrow().is_leaf {
                    child.borrow().construct(remaining, root) || root.construct(remaining, root)
                } else {
                    child.borrow().construct(remaining, root)
                }
            } else {
                false
            }
        }
    }
}

impl Solution for Day19 {
    fn test_input() -> String {
        String::from(
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        )
    }

    fn solve_part_1(input: String) -> String {
        let lines = &mut input.lines();
        let mut trie = Node::new();
        for word in lines.next().unwrap().split(",").map(|w| w.trim()) {
            let word: Vec<char> = word.chars().collect();
            trie.insert(&word);
        }
        // trie.print(0);
        lines
            .skip(1)
            .enumerate()
            .filter(|(i, line)| {
                println!("{}th", i);
                trie.construct(line, &trie)
            })
            .count()
            .to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day19::test_input();
        let ans = Day19::solve_part_1(input);
        assert_eq!(ans, "6");
    }

    #[test]
    fn test_part_2() {
        let input = Day19::test_input();
        let ans = Day19::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
