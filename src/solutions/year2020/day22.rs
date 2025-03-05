use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

type GameResult = bool;

const PLAYER_1_WINS: GameResult = true;

struct Game {
    p1: VecDeque<i64>,
    p2: VecDeque<i64>,
    history: HashSet<(VecDeque<i64>, VecDeque<i64>)>,
    cache: Rc<RefCell<HashMap<(VecDeque<i64>, VecDeque<i64>), GameResult>>>,
}

impl Game {
    fn new(input: String) -> Self {
        let mut p1: VecDeque<i64> = VecDeque::new();
        let mut p2: VecDeque<i64> = VecDeque::new();
        let lines = &mut input.lines();
        for line in lines.skip(1) {
            if line == "" {
                break;
            }
            p1.push_back(line.parse().unwrap());
        }
        for line in lines.skip(1) {
            p2.push_back(line.parse().unwrap());
        }
        let history = HashSet::new();

        let cache = Rc::new(RefCell::new(HashMap::new()));
        Game {
            p1,
            p2,
            history,
            cache,
        }
    }

    fn calculate_score(&self, result: GameResult) -> i64 {
        let winner = if result == PLAYER_1_WINS {
            &self.p1
        } else {
            &self.p2
        };
        let score = winner
            .iter()
            .rev()
            .enumerate()
            .map(|(index, card)| (index as i64 + 1) * card)
            .sum::<i64>();
        score
    }

    fn simulate_with_score(&mut self) -> i64 {
        let result = self.simulate();
        self.calculate_score(result)
    }

    fn simulate(&mut self) -> GameResult {
        loop {
            if let Some(result) = self.tick() {
                break result;
            }
        }
    }

    fn tick(&mut self) -> Option<GameResult> {
        if self.check_dejavu() {
            return Some(PLAYER_1_WINS);
        }

        let (c1, c2) = self.deal_cards();

        let l1 = self.p1.len() as i64;
        let l2 = self.p2.len() as i64;

        let can_spawn_game = l1 >= c1 && l2 >= c2;
        let result = if can_spawn_game {
            self.spawn_game(c1, c2)
        } else {
            c1 > c2
        };

        let (winner, c1, c2) = if result == PLAYER_1_WINS {
            (&mut self.p1, c1, c2)
        } else {
            (&mut self.p2, c2, c1)
        };

        winner.push_back(c1);
        winner.push_back(c2);

        if let Some(result) = self.check_ended() {
            Some(result)
        } else {
            None
        }
    }

    fn spawn_game(&mut self, c1: i64, c2: i64) -> GameResult {
        let p1: VecDeque<i64> = self.p1.to_owned().into_iter().take(c1 as usize).collect();
        let p2: VecDeque<i64> = self.p2.to_owned().into_iter().take(c2 as usize).collect();
        if let Some(cached) = self.cache.borrow().get(&(p1.clone(), p2.clone())) {
            return *cached;
        }
        let key = (p1.clone(), p2.clone());
        let mut spawned_game = Game {
            p1,
            p2,
            history: HashSet::new(),
            cache: self.cache.clone(),
        };
        let result = spawned_game.simulate();
        self.cache.borrow_mut().insert(key, result);
        result
    }

    fn check_ended(&self) -> Option<GameResult> {
        if self.p1.len() == 0 {
            Some(!PLAYER_1_WINS)
        } else if self.p2.len() == 0 {
            Some(PLAYER_1_WINS)
        } else {
            None
        }
    }

    fn check_dejavu(&mut self) -> bool {
        let config = (self.p1.to_owned(), self.p2.to_owned());
        !self.history.insert(config)
    }

    fn deal_cards(&mut self) -> (i64, i64) {
        (self.p1.pop_front().unwrap(), self.p2.pop_front().unwrap())
    }
}

use crate::Solution;

pub struct Day22 {}

impl Solution for Day22 {
    fn test_input() -> String {
        String::from(
            "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10",
        )
    }
    fn solve_part_1(input: String) -> String {
        let mut p1: VecDeque<i64> = VecDeque::new();
        let mut p2: VecDeque<i64> = VecDeque::new();
        let lines = &mut input.lines();
        for line in lines.skip(1) {
            if line == "" {
                break;
            }
            p1.push_back(line.parse().unwrap());
        }
        for line in lines.skip(1) {
            p2.push_back(line.parse().unwrap());
        }
        while let Some(c1) = p1.pop_front() {
            if let Some(c2) = p2.pop_front() {
                if c1 > c2 {
                    p1.push_back(c1);
                    p1.push_back(c2);
                } else {
                    p2.push_back(c2);
                    p2.push_back(c1);
                }
            } else {
                p1.push_front(c1);
            }
        }
        let winner = if p1.len() == 0 { p2 } else { p1 };
        let score = winner
            .iter()
            .rev()
            .enumerate()
            .map(|(index, card)| (index as i64 + 1) * card)
            .sum::<i64>();
        score.to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut game = Game::new(input);
        let score = game.simulate_with_score();
        score.to_string()
    }
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day22::test_input();
        let ans = Day22::solve_part_1(input);
        assert_eq!(ans, "306")
    }

    #[test]
    fn test_part_2() {
        let input = Day22::test_input();
        let ans = Day22::solve_part_2(input);
        assert_eq!(ans, "291")
    }
}
