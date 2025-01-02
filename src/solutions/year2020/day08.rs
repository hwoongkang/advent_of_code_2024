use crate::Solution;

pub struct Day08;

#[derive(Clone, PartialEq, Eq)]
enum Cmd {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

use Cmd::*;

struct State {
    acc: i32,
    clock: usize,
    cmds: Vec<Cmd>,
    visited: Vec<bool>,
}

impl State {
    fn new(lines: &mut std::str::Lines) -> Self {
        let cmds: Vec<Cmd> = lines
            .map(|line| {
                let mut words = line.split_ascii_whitespace();
                let name = words.next().unwrap();
                let arg = words.next().unwrap();
                let plus = arg.chars().nth(0).unwrap() == '+';
                let sign = if plus { 1 } else { -1 };
                let arg = &arg[1..].parse().unwrap();
                match name {
                    "nop" => Nop(arg * sign),
                    "acc" => Acc(arg * sign),
                    "jmp" => Jmp(arg * sign),
                    _ => unreachable!(),
                }
            })
            .collect();
        Self {
            acc: 0,
            clock: 0,
            visited: vec![false; cmds.len()],
            cmds,
        }
    }
    fn tick(&mut self) -> Option<i32> {
        match &self.cmds[self.clock] {
            Nop(_) => self.clock += 1,
            Acc(by) => {
                self.acc += by;
                self.clock += 1
            }
            Jmp(step) => self.clock = ((self.clock as i32) + step) as usize,
        }

        let ans = if self.visited[self.clock] {
            Some(self.acc)
        } else {
            None
        };
        self.visited[self.clock] = true;
        ans
    }

    fn reset(&mut self) {
        self.clock = 0;
        self.acc = 0;
    }
    fn simulate_loop(&mut self, program: &[Cmd]) -> Option<i32> {
        self.reset();
        let len = program.len();
        let mut visited = vec![false; len];
        visited[self.clock] = true;

        loop {
            match &program[self.clock] {
                Nop(_) => self.clock += 1,
                Acc(by) => {
                    self.acc += by;
                    self.clock += 1;
                }
                Jmp(step) => self.clock = ((self.clock as i32) + step) as usize,
            }
            if self.clock >= len {
                break Some(self.acc);
            } else if visited[self.clock] {
                break None;
            }
            visited[self.clock] = true;
        }
    }
}

impl Solution for Day08 {
    fn test_input() -> String {
        String::from(
            "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6",
        )
    }

    fn solve_part_1(_input: String) -> String {
        let mut state: State = State::new(&mut _input.lines());
        loop {
            if let Some(val) = state.tick() {
                break val;
            }
        }
        .to_string()
    }

    fn solve_part_2(_input: String) -> String {
        let mut state = State::new(&mut _input.lines());
        let program = state.cmds.clone();
        for i in 0..program.len() {
            let mut new_program = program.clone();
            match &program[i] {
                Acc(_) => continue,
                Jmp(n) => {
                    new_program[i] = Nop(*n);
                    if let Some(val) = state.simulate_loop(&new_program) {
                        return val.to_string();
                    }
                }
                Nop(n) => {
                    new_program[i] = Jmp(*n);
                    if let Some(val) = state.simulate_loop(&new_program) {
                        return val.to_string();
                    }
                }
            }
        }
        String::from("0")
    }
}

#[cfg(test)]
mod day08_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day08::test_input();
        let ans = Day08::solve_part_1(input);
        assert_eq!(ans, "5");
    }

    #[test]
    fn test_part_2() {
        let input = Day08::test_input();
        let ans = Day08::solve_part_2(input);
        assert_eq!(ans, "8");
    }
}
