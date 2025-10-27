use std::{cell::RefCell, rc::Rc};

use crate::Solution;

type Value = usize;

struct Node {
    value: Value,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn get_next(&self) -> Rc<RefCell<Node>> {
        if let Some(next) = self.next.clone() {
            next.clone()
        } else {
            panic!("Node initialization error");
        }
    }
}

pub struct Day23 {}

fn tick(nodes: &[Rc<RefCell<Node>>], head: usize) -> usize {
    let len = nodes.len();
    let cut_start = nodes[head - 1].clone();
    let partial_start = cut_start.borrow().get_next();
    let partial_end = partial_start.borrow().get_next().borrow().get_next();
    let cut_end = partial_end.borrow().get_next();

    let mut scan_head = partial_start.clone();
    let mut seen_values = vec![];

    loop {
        seen_values.push(scan_head.borrow().value);
        let next = scan_head.borrow().get_next();

        scan_head = next;
        if scan_head.borrow().value == cut_end.borrow().value {
            break;
        }
    }
    let current_value = cut_start.borrow().value;
    let mut target_value = current_value;
    loop {
        target_value -= 1;
        if target_value == 0 {
            target_value = len;
        }
        if !seen_values.contains(&target_value) {
            break;
        }
    }

    let insert_start = nodes[target_value - 1].clone();
    let insert_end = insert_start.borrow().get_next();

    cut_start.borrow_mut().next = Some(cut_end.clone());
    cut_end.borrow_mut().prev = Some(cut_start.clone());

    partial_start.borrow_mut().prev = Some(insert_start.clone());
    insert_start.borrow_mut().next = Some(partial_start.clone());

    partial_end.borrow_mut().next = Some(insert_end.clone());
    insert_end.borrow_mut().prev = Some(partial_end.clone());
    let next_head = cut_end.borrow().value;
    next_head
}

impl Solution for Day23 {
    fn test_input() -> String {
        String::from("389125467")
    }
    fn solve_part_1(input: String) -> String {
        let values: Vec<Value> = input
            .chars()
            .map(|n| n.to_string().as_str().parse().unwrap())
            .collect();
        let nodes: Vec<Rc<RefCell<Node>>> = (1..=values.len())
            .map(|value| Node {
                value,
                next: None,
                prev: None,
            })
            .map(|node| Rc::new(RefCell::new(node)))
            .collect();
        let len = nodes.len();
        for i in 0..len {
            let me = values[i];
            let mut node = nodes[me - 1].borrow_mut();
            let prev = (i + len - 1) % len;
            let prev = values[prev];
            let prev = nodes[prev - 1].clone();
            node.prev = Some(prev);
            let next = (i + 1) % len;
            let next = values[next];
            let next = nodes[next - 1].clone();
            node.next = Some(next);
        }

        let mut head = values[0];

        for _it in 0..100 {
            head = tick(&nodes, head);
        }

        let target = 1;
        let mut scan_head = nodes[0].clone();
        let mut answer = String::new();
        loop {
            let next = scan_head.borrow().get_next();
            if next.borrow().value == target {
                break;
            }
            answer += next.borrow().value.to_string().as_str();
            scan_head = next;
        }

        answer
    }
    fn solve_part_2(input: String) -> String {
        let mut values: Vec<Value> = input
            .chars()
            .map(|n| n.to_string().as_str().parse().unwrap())
            .collect();
        for i in 10..=1_000_000 {
            values.push(i);
        }
        let nodes: Vec<Rc<RefCell<Node>>> = (1..=1_000_000)
            .map(|value| Node {
                value,
                next: None,
                prev: None,
            })
            .map(|node| Rc::new(RefCell::new(node)))
            .collect();
        let len = nodes.len();
        for i in 0..len {
            let me = values[i];
            let mut node = nodes[me - 1].borrow_mut();
            let prev = (i + len - 1) % len;
            let prev = values[prev];
            let prev = nodes[prev - 1].clone();
            node.prev = Some(prev);
            let next = (i + 1) % len;
            let next = values[next];
            let next = nodes[next - 1].clone();
            node.next = Some(next);
        }

        let mut head = values[0];
        for i in 0..10_000_000 {
            if i % 1_000_000 == 0 {
                println!("{}th iteration", i);
            }
            head = tick(&nodes, head);
        }
        let head = nodes[0].clone();
        let i = head.borrow().get_next();
        let j = i.borrow().get_next();
        let i = i.borrow().value;
        let j = j.borrow().value;
        (i * j).to_string()
    }
}

#[cfg(test)]
mod day23_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day23::test_input();
        let ans = Day23::solve_part_1(input);
        assert_eq!(ans, "67384529")
    }

    #[test]
    fn test_part_2() {
        let input = Day23::test_input();
        let ans = Day23::solve_part_2(input);
        assert_eq!(ans, "149245887792")
    }
}
