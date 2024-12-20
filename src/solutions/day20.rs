use std::collections::{BinaryHeap, VecDeque};

use super::Solution;

#[derive(Clone)]
struct Maze {
    map: Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Maze {
    fn from(input: String) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let map = input
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .map(|(c, ch)| match ch {
                        '#' => false,
                        '.' => true,
                        'S' => {
                            start = (r, c);
                            true
                        }
                        'E' => {
                            end = (r, c);
                            true
                        }
                        _ => unimplemented!(),
                    })
                    .collect()
            })
            .collect();
        Self { map, start, end }
    }

    fn size(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }

    fn _standard(&self) -> usize {
        let mut queue = VecDeque::from([(self.start, 0)]);
        let (mr, mc) = self.size();
        let mut visited = vec![vec![false; mc]; mr];
        visited[self.start.0][self.start.1] = true;
        while let Some(((r, c), cost)) = queue.pop_front() {
            if (r, c) == self.end {
                return cost;
            }
            let ir = r as i32;
            let ic = c as i32;
            for (dr, dc) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let ir = ir + dr;
                let ic = ic + dc;
                let r = ir as usize;
                let c = ic as usize;
                if !self.map[r][c] {
                    continue;
                }
                if visited[r][c] {
                    continue;
                }
                visited[r][c] = true;
                queue.push_back(((r, c), cost + 1));
            }
        }

        0
    }

    fn trajectory(&self) -> Vec<(usize, usize)> {
        let (mr, mc) = self.size();
        let mut visited = vec![vec![false; mc]; mr];
        visited[self.start.0][self.start.1] = true;
        let mut queue = VecDeque::from([(self.start, vec![self.start])]);
        while let Some((pos, history)) = queue.pop_front() {
            if pos == self.end {
                return history;
            }
            let (r, c) = pos;
            let ir = r as i32;
            let ic = c as i32;
            for (dr, dc) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let ir = ir + dr;
                let ic = ic + dc;
                let r = ir as usize;
                let c = ic as usize;
                if !self.map[r][c] {
                    continue;
                }
                if visited[r][c] {
                    continue;
                }
                visited[r][c] = true;
                let mut h = history.clone();
                h.push((r, c));
                queue.push_back(((r, c), h));
            }
        }
        vec![]
    }
}

#[derive(PartialEq, Eq)]
struct State {
    cost: usize,
    pos: (usize, usize),
    cheat_time: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn cheat_even_more(input: String, threshold: usize, max_cheat_time: usize) -> String {
    let maze = Maze::from(input);
    let (mr, mc) = maze.size();
    let mut costs = vec![vec![usize::MAX; mc]; mr];
    let traj = maze.trajectory();
    for (cost, &(r, c)) in traj.iter().enumerate() {
        costs[r][c] = cost;
    }

    let mut ans = 0;

    for (r, c) in traj {
        let mut dists = vec![vec![usize::MAX; mc]; mr];
        dists[r][c] = costs[r][c];
        let initial_state = State {
            pos: (r, c),
            cost: costs[r][c],
            cheat_time: 0,
        };
        let mut heap = BinaryHeap::from([initial_state]);

        while let Some(state) = heap.pop() {
            let State {
                pos: (r, c),
                cost,
                cheat_time,
            } = state;

            if dists[r][c] < cost {
                continue;
            }
            let time_saved = costs[r][c].saturating_sub(cost);
            if maze.map[r][c] && time_saved >= threshold {
                ans += 1;
            }
            let ir = r as i32;
            let ic = c as i32;
            for (dr, dc) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let ir = ir + dr;
                let ic = ic + dc;
                if ir < 0 || ic < 0 {
                    continue;
                }
                let r = ir as usize;
                let c = ic as usize;
                if r >= mr || c >= mc {
                    continue;
                }
                let cost = cost + 1;
                let cheat_time = cheat_time + 1;
                if cheat_time > max_cheat_time {
                    continue;
                }
                if dists[r][c] <= cost {
                    continue;
                }
                dists[r][c] = cost;
                heap.push(State {
                    pos: (r, c),
                    cost,
                    cheat_time,
                })
            }
        }
    }

    ans.to_string()
}
pub struct Day20;

impl Solution for Day20 {
    fn test_input() -> String {
        String::from(
            "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
        )
    }

    fn solve_part_1(input: String) -> String {
        cheat_even_more(input, 100, 2)
    }

    fn solve_part_2(input: String) -> String {
        cheat_even_more(input, 100, 20)
    }
}

#[cfg(test)]
mod day20_tests {
    use super::*;

    #[test]
    fn test_standard() {
        let input = Day20::test_input();
        let maze = Maze::from(input);
        assert_eq!(maze._standard(), 84);
        assert_eq!(maze._standard(), maze.trajectory().len() - 1)
    }

    #[test]
    fn test_part_1() {
        let input = Day20::test_input();
        let ans = cheat_even_more(input, 50, 2);
        assert_eq!(ans, "16");
    }

    #[test]
    fn test_part_2() {
        let input = Day20::test_input();
        let ans = cheat_even_more(input, 50, 20);
        assert_eq!(ans, "285");
    }
}
