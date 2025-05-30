use std::collections::{HashMap, HashSet};

use crate::Solution;

fn part_2_nth(input: String, nth: usize) -> (i32, i32) {
    let mut asteroids = vec![];
    for (r, line) in input.lines().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if char == '#' {
                asteroids.push((r as i32, c as i32))
            }
        }
    }
    let l = asteroids.len();
    let mut ans = (0, 0);
    for i in 0..l {
        let mut set = HashSet::new();
        for j in 0..l {
            if i == j {
                continue;
            }
            let dx = asteroids[j].0 - asteroids[i].0;
            let dy = asteroids[j].1 - asteroids[i].1;
            let d = gcd(dx.abs(), dy.abs());
            set.insert((dx / d, dy / d));
        }

        let max_visible = set.len();
        if max_visible > ans.0 {
            ans = (max_visible, i);
        }
    }

    // println!("best position: {:?}", asteroids[ans.1]);
    let best = ans.1;
    let mut map = HashMap::new();
    for i in 0..l {
        if i == best {
            continue;
        }
        let dx = asteroids[i].0 - asteroids[best].0;
        let dy = asteroids[i].1 - asteroids[best].1;
        let d = gcd(dx.abs(), dy.abs());
        let key = (dx / d, dy / d);
        let v = map.entry(key).or_insert(vec![]);
        v.push((d, i));
        v.sort_by(|a, b| b.0.cmp(&a.0));
    }
    // println!("{:?}", map);
    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by(|a, b| {
        let atan2 = |tup: (i32, i32)| -> f64 {
            let dc = tup.1;
            let dr = tup.0;
            (dc as f64).atan2(dr as f64)
        };
        atan2(b.0).partial_cmp(&atan2(a.0)).unwrap()
    });
    // println!("{:?}", v);

    let mut index = 0;
    let mut removed = 0;
    loop {
        let asteroid_index = v[index].1.pop().unwrap();
        removed += 1;
        let asteroid = asteroids[asteroid_index.1];
        // println!("#{}. removing ({}, {}) ", removed, asteroid.0, asteroid.1);
        if removed == nth {
            break asteroid;
        }
        if v[index].1.len() == 0 {
            v.remove(index);
        } else {
            index += 1;
            index %= v.len();
        }
    }

    // atan2
    // -x, +y <= pi
    // +x, +y <=pi/2
    // +x, -y <= 0
    // -x, -y <=-pi/2

    // us:
    // -r, +c -> 1
    // +r, +c -> 2,
    // +r, -c -> 3
    // -r, -c -> 4

    // dx = dr
    // dy = dc
}

fn gcd(a: i32, b: i32) -> i32 {
    if a > b {
        gcd(b, a)
    } else {
        if a == 0 {
            return b;
        }
        let d = b % a;
        if d == 0 {
            a
        } else {
            gcd(d, a)
        }
    }
}

pub struct Day10 {}

impl Solution for Day10 {
    fn test_input() -> String {
        String::new()
    }
    fn solve_part_1(input: String) -> String {
        let mut asteroids = vec![];
        for (r, line) in input.lines().enumerate() {
            for (c, char) in line.chars().enumerate() {
                if char == '#' {
                    asteroids.push((r as i32, c as i32))
                }
            }
        }
        let l = asteroids.len();
        let mut ans = 0;
        for i in 0..l {
            let mut set = HashSet::new();
            for j in 0..l {
                if i == j {
                    continue;
                }
                let dx = asteroids[j].0 - asteroids[i].0;
                let dy = asteroids[j].1 - asteroids[i].1;
                let d = gcd(dx.abs(), dy.abs());
                set.insert((dx / d, dy / d));
            }

            ans = ans.max(set.len())
        }
        ans.to_string()
    }
    fn solve_part_2(input: String) -> String {
        let (y, x) = part_2_nth(input, 200);
        (100 * x + y).to_string()
    }
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = String::from(
            ".#..#
.....
#####
....#
...##",
        );

        let ans = Day10::solve_part_1(input);
        assert_eq!(ans, "8");

        let input = String::from(
            "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####",
        );

        let ans = Day10::solve_part_1(input);
        assert_eq!(ans, "33");

        let input = String::from(
            ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
        );

        let ans = Day10::solve_part_1(input);
        assert_eq!(ans, "210")
    }

    #[test]
    fn test_part_2() {
        let input = String::from(
            ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##",
        );
        assert_eq!(part_2_nth(input, 5), (2, 9));

        let input = String::from(
            ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
        );
        assert_eq!(part_2_nth(input.clone(), 200), (2, 8));
        assert_eq!(Day10::solve_part_2(input), "802")
    }
}
