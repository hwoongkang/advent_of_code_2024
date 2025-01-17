use crate::Solution;

pub struct Day14;

fn parse_robot(line: &str) -> ((i32, i32), (i32, i32)) {
    let mut words = line.split_ascii_whitespace();
    let mut p = words.next().unwrap().split("=").nth(1).unwrap().split(",");
    let px = p.next().unwrap().parse().unwrap();
    let py = p.next().unwrap().parse().unwrap();
    let mut v = words.next().unwrap().split("=").nth(1).unwrap().split(",");
    let vx = v.next().unwrap().parse().unwrap();
    let vy = v.next().unwrap().parse().unwrap();
    ((px, py), (vx, vy))
}

fn solve_part_1(input: String, bound: (usize, usize)) -> String {
    let mut robots = vec![vec![0; bound.1]; bound.0];
    let b = (bound.0 as i32, bound.1 as i32);
    for line in input.lines() {
        let (mut p, v) = parse_robot(line);
        for _ in 0..100 {
            p.0 += v.0;
            p.0 = p.0.rem_euclid(b.0);
            p.1 += v.1;
            p.1 = p.1.rem_euclid(b.1);
        }
        robots[p.0 as usize][p.1 as usize] += 1;
    }
    let h = (bound.0 / 2, bound.1 / 2);
    let mut ans = 1;
    let mut temp = 0;
    for i in 0..h.0 {
        for j in 0..h.1 {
            temp += robots[i][j];
        }
    }
    ans *= temp;

    temp = 0;
    for i in 0..h.0 {
        for j in (h.1 + 1)..bound.1 {
            temp += robots[i][j];
        }
    }
    ans *= temp;

    temp = 0;
    for i in (h.0 + 1)..bound.0 {
        for j in 0..h.1 {
            temp += robots[i][j];
        }
    }
    ans *= temp;

    temp = 0;
    for i in (h.0 + 1)..bound.0 {
        for j in (h.1 + 1)..bound.1 {
            temp += robots[i][j];
        }
    }
    ans *= temp;

    ans.to_string()
}

impl Solution for Day14 {
    fn test_input() -> String {
        String::from(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
        )
    }

    fn solve_part_1(input: String) -> String {
        solve_part_1(input, (101, 103))
    }

    fn solve_part_2(input: String) -> String {
        let mut robots = input.lines().map(parse_robot).collect::<Vec<_>>();
        let b = (101, 103);
        let print = |index: usize, robots: &Vec<((i32, i32), (i32, i32))>| {
            let mut v = vec![vec![0; 103]; 101];
            for r in robots.iter() {
                v[r.0 .0 as usize][r.0 .1 as usize] += 1;
            }
            for row in v {
                for n in row {
                    if n > 0 {
                        print!("*");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!("{}", index);
        };
        let tick = |robots: &mut Vec<((i32, i32), (i32, i32))>| {
            for robot in robots.iter_mut() {
                let (p, v) = robot;
                p.0 += v.0;
                p.0 = p.0.rem_euclid(b.0);
                p.1 += v.1;
                p.1 = p.1.rem_euclid(b.1);
            }
        };
        let score = |robots: &Vec<((i32, i32), (i32, i32))>| -> usize {
            let mut v = vec![vec![0; 103]; 101];
            for r in robots.iter() {
                v[r.0 .0 as usize][r.0 .1 as usize] += 1;
            }
            let robots = v;
            let mut ans = 1;
            let mut temp = 0;
            let bound = (101, 103);
            let h = (bound.0 / 2, bound.1 / 2);
            for i in 0..h.0 {
                for j in 0..h.1 {
                    temp += robots[i][j];
                }
            }
            ans *= temp;

            temp = 0;
            for i in 0..h.0 {
                for j in (h.1 + 1)..bound.1 {
                    temp += robots[i][j];
                }
            }
            ans *= temp;

            temp = 0;
            for i in (h.0 + 1)..bound.0 {
                for j in 0..h.1 {
                    temp += robots[i][j];
                }
            }
            ans *= temp;

            temp = 0;
            for i in (h.0 + 1)..bound.0 {
                for j in (h.1 + 1)..bound.1 {
                    temp += robots[i][j];
                }
            }
            ans *= temp;

            ans
        };
        let mut index = 0;
        let mut prev_max = usize::MAX;

        loop {
            let mut l = String::new();
            let s = score(&robots);
            if s <= prev_max {
                prev_max = s;
                print(index, &robots);
                println!("{}", s);
                let _ = std::io::stdin().read_line(&mut l);
            } else {
                //   println!("{}", index);
            }
            index += 1;
            tick(&mut robots);
        }
    }
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day14::test_input();
        let ans = solve_part_1(input, (11, 7));
        assert_eq!(ans, "12");
    }

    #[test]
    fn test_part_2() {
        assert!(true);
    }
}
