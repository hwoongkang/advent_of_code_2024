use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::Solution;

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

    fn matches(&self, word: &str) -> Vec<usize> {
        fn helper(node: &Node, word: &str, sofar: usize, v: &mut Vec<usize>) {
            if node.is_leaf {
                v.push(sofar)
            }
            if word == "" {
                return;
            }
            let key = word.chars().next().unwrap();
            let remaining = &word[1..];
            let child = node.children.get(&key);
            if let Some(child) = child {
                helper(&child.borrow(), remaining, sofar + 1, v);
            }
        }
        let mut ans = vec![];
        helper(self, word, 0, &mut ans);
        ans
    }
}

fn constructions(trie: &Node, word: &str) -> usize {
    let l = word.len();
    let mut dp = vec![0usize; l + 1];
    dp[0] = 1;
    for i in 0..=l {
        let count = dp[i];
        if count == 0 {
            continue;
        }
        let word = &word[i..];
        let matches = trie.matches(word);
        for delta in matches {
            let j = i + delta;
            if j <= l {
                dp[j] += count;
            }
        }
    }

    dp[l]
}

fn can_construct(trie: &Node, word: &str) -> bool {
    constructions(trie, word) != 0
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
            .filter(|line| can_construct(&trie, line))
            .count()
            .to_string()
    }

    fn solve_part_2(input: String) -> String {
        let lines = &mut input.lines();
        let mut trie = Node::new();
        for word in lines.next().unwrap().split(",").map(|w| w.trim()) {
            let word: Vec<char> = word.chars().collect();
            trie.insert(&word);
        }
        // trie.print(0);
        lines
            .skip(1)
            .map(|line| constructions(&trie, line))
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    #[test]
    fn test_matches() {
        let mut trie = Node::new();
        trie.insert(&['r']);
        trie.insert(&['r', 'r']);
        trie.insert(&['r', 'r', 'r', 'r']);
        trie.insert(&['r', 'r', 'r', 'r', 'w']);
        let matches = trie.matches("rrrrw");
        assert_eq!(matches, [1, 2, 4, 5]);
        let matches = trie.matches("wwrr");
        assert_eq!(matches, []);
        let matches = trie.matches("rrr");
        assert_eq!(matches, [1, 2]);
    }

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
        assert_eq!(ans, "16");
    }
}
