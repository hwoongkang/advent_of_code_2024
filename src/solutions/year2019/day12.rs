use std::collections::HashMap;

use crate::Solution;

#[derive(Clone, Copy)]
struct Vec3d(i64, i64, i64);

impl Vec3d {
    fn signum(&self) -> Vec3d {
        Vec3d(self.0.signum(), self.1.signum(), self.2.signum())
    }

    fn absolute_norm(&self) -> i64 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl std::ops::Neg for Vec3d {
    type Output = Vec3d;
    fn neg(self) -> Self::Output {
        Vec3d(-self.0, -self.1, -self.2)
    }
}

impl std::ops::Add for Vec3d {
    type Output = Vec3d;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3d(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::AddAssign for Vec3d {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Vec3d {
    type Output = Vec3d;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

#[derive(Clone, Copy)]
struct Moon {
    p: Vec3d,
    v: Vec3d,
}

impl Moon {
    fn from(line: &str) -> Self {
        let words = &mut line[1..line.len() - 1].split(",");
        let parse_word = |words: &mut dyn Iterator<Item = &str>| -> i64 {
            words
                .next()
                .unwrap()
                .trim()
                .split("=")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap()
        };
        let x = parse_word(words);
        let y = parse_word(words);
        let z = parse_word(words);
        let p = Vec3d(x, y, z);
        let v = Vec3d(0, 0, 0);
        Self { p, v }
    }

    fn gravity(&mut self, rhs: &Moon) {
        let delta = (rhs.p - self.p).signum();
        self.v += delta;
    }

    fn update_position(&mut self) {
        self.p += self.v;
    }

    fn energy(&self) -> i64 {
        let potential = self.p.absolute_norm();
        let kinetic = self.v.absolute_norm();
        potential * kinetic
    }
}

fn simulate_part_1(input: String, steps: i64) -> i64 {
    let mut moons: Vec<Moon> = input.lines().map(Moon::from).collect();
    let num = moons.len();
    for _ in 0..steps {
        for i in 0..num {
            let (first_half, second_half) = moons.split_at_mut(i + 1);
            for j in i + 1..num {
                let diff = j - i - 1;
                let a = &mut first_half[i];
                let b = &mut second_half[diff];
                a.gravity(&b);
                b.gravity(&a);
            }
        }
        for moon in moons.iter_mut() {
            moon.update_position();
        }
    }
    moons.into_iter().map(|moon| moon.energy()).sum()
}

fn one_dimensional(mut ps: Vec<i64>, mut vs: Vec<i64>) -> (usize, usize) {
    type Key = (i64, i64, i64, i64, i64, i64, i64, i64);
    let hash = |ps: &[i64], vs: &[i64]| -> Key {
        (ps[0], ps[1], ps[2], ps[3], vs[0], vs[1], vs[2], vs[3])
    };
    let mut history: HashMap<Key, usize> = HashMap::new();
    history.insert(hash(&ps, &vs), 0);
    for t in 0.. {
        // velocity update
        for i in 0..4 {
            let (first, second) = vs.split_at_mut(i + 1);
            for j in i + 1..4 {
                let diff = j - i - 1;
                let pa = ps[i];
                let va = &mut first[i];

                let pb = ps[j];
                let vb = &mut second[diff];

                let delta_a = (pb - pa).signum();
                *va += delta_a;

                let delta_b = (pa - pb).signum();
                *vb += delta_b;
            }
        }
        // position update
        for i in 0..4 {
            ps[i] += vs[i];
        }
        // history check
        let now = t + 1;
        let key = hash(&ps, &vs);

        if let Some(past) = history.get(&key) {
            return (*past, now - past);
        } else {
            history.insert(key, now);
        }
    }
    (0, 0)
}

pub struct Day12 {}

fn parse(input: String) -> Vec<Moon> {
    input.lines().map(Moon::from).collect()
}

fn gcd(a: usize, b: usize) -> usize {
    if !(a < b) {
        gcd(b, a)
    } else {
        let d = b % a;
        if d == 0 {
            a
        } else {
            gcd(d, a)
        }
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

impl Solution for Day12 {
    fn test_input() -> String {
        String::from(
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",
        )
    }
    fn solve_part_1(input: String) -> String {
        simulate_part_1(input, 1000).to_string()
    }
    fn solve_part_2(input: String) -> String {
        let moons = parse(input);

        let x_pos: Vec<i64> = moons.iter().map(|moon| moon.p.0).collect();
        let x_vel: Vec<i64> = moons.iter().map(|moon| moon.v.0).collect();

        let (_, x_period) = one_dimensional(x_pos, x_vel);
        let y_pos: Vec<i64> = moons.iter().map(|moon| moon.p.1).collect();
        let y_vel: Vec<i64> = moons.iter().map(|moon| moon.v.1).collect();

        let (_, y_period) = one_dimensional(y_pos, y_vel);

        let z_pos: Vec<i64> = moons.iter().map(|moon| moon.p.2).collect();
        let z_vel: Vec<i64> = moons.iter().map(|moon| moon.v.2).collect();
        let (_, z_period) = one_dimensional(z_pos, z_vel);
        let ans = lcm(x_period, y_period);
        let ans = lcm(ans, z_period);
        ans.to_string()
    }
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day12::test_input();
        let ans = simulate_part_1(input, 10);
        assert_eq!(ans, 179);

        let input = String::from(
            "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>",
        );
        let ans = simulate_part_1(input, 100);
        assert_eq!(ans, 1940)
    }

    #[test]
    fn test_part_2_basic() {
        let input = Day12::test_input();
        let moons = parse(input);

        let x_pos: Vec<i64> = moons.iter().map(|moon| moon.p.0).collect();
        let x_vel: Vec<i64> = moons.iter().map(|moon| moon.v.0).collect();

        println!("{:?}", one_dimensional(x_pos, x_vel));
        let y_pos: Vec<i64> = moons.iter().map(|moon| moon.p.1).collect();
        let y_vel: Vec<i64> = moons.iter().map(|moon| moon.v.1).collect();

        println!("{:?}", one_dimensional(y_pos, y_vel));

        let z_pos: Vec<i64> = moons.iter().map(|moon| moon.p.2).collect();
        let z_vel: Vec<i64> = moons.iter().map(|moon| moon.v.2).collect();
        println!("{:?}", one_dimensional(z_pos, z_vel));
        assert!(false);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(lcm(18, 28), 44), 2772);
    }
}
