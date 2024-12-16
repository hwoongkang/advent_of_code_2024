use super::Solution;

pub struct Day13;

fn parse_button(line: &str) -> (i64, i64) {
    let line = line.split(":").nth(1).unwrap();
    let mut xy = line.split(",");
    let x = xy
        .next()
        .unwrap()
        .split("+")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let y = xy
        .next()
        .unwrap()
        .split("+")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    (x, y)
}

fn parse_prize(line: &str) -> (i64, i64) {
    let line = line.split(":").nth(1).unwrap();
    let mut xy = line.split(",");
    let x = xy
        .next()
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let y = xy
        .next()
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    (x, y)
}

fn parse_input(lines: &mut std::str::Lines) -> Option<((i64, i64), (i64, i64), (i64, i64))> {
    let line = lines.next();
    if line.is_none() {
        return None;
    }
    let a = parse_button(line.unwrap());
    let b = parse_button(lines.next().unwrap());
    let prize = parse_prize(lines.next().unwrap());
    lines.next();
    Some((a, b, prize))
}

impl Solution for Day13 {
    fn test_input() -> String {
        String::from(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        )
    }

    fn solve_part_1(_input: String) -> String {
        let input = _input + "\n\n";
        let lines = &mut input.lines();
        let mut ans = 0;
        while let Some((a, b, p)) = parse_input(lines) {
            // a.0 * na + b.0 * nb = p.0
            // a.1 * na + b.1 * nb = p.1
            // a.0 b.0  na  =  p.0
            // a.1 b.1  nb     p.1
            // na =  b.1 -b.0  p.0
            // nb   -a.1  a.0  p.1
            let mut det = a.0 * b.1 - a.1 * b.0;

            if det == 0 {
                panic!("{:?} {:?}", a, b)
            }
            let mut na = b.1 * p.0 - b.0 * p.1;
            let mut nb = -a.1 * p.0 + a.0 * p.1;
            if det < 0 {
                na *= -1;
                nb *= -1;
                det *= -1;
            }
            let rema = na % det;
            let remb = nb % det;

            if rema != 0 || remb != 0 {
                continue;
            }
            na /= det;
            nb /= det;
            ans += 3 * na + nb;
        }
        ans.to_string()
    }

    fn solve_part_2(_input: String) -> String {
        let input = _input + "\n\n";
        let lines = &mut input.lines();
        let mut ans = 0;
        while let Some((a, b, p)) = parse_input(lines) {
            let p = (p.0 + 10000000000000, p.1 + 10000000000000);
            // a.0 * na + b.0 * nb = p.0
            // a.1 * na + b.1 * nb = p.1
            // a.0 b.0  na  =  p.0
            // a.1 b.1  nb     p.1
            // na =  b.1 -b.0  p.0
            // nb   -a.1  a.0  p.1
            let mut det = a.0 * b.1 - a.1 * b.0;

            if det == 0 {
                panic!("{:?} {:?}", a, b)
            }
            let mut na = b.1 * p.0 - b.0 * p.1;
            let mut nb = -a.1 * p.0 + a.0 * p.1;
            if det < 0 {
                na *= -1;
                nb *= -1;
                det *= -1;
            }
            let rema = na % det;
            let remb = nb % det;

            if rema != 0 || remb != 0 {
                continue;
            }
            na /= det;
            nb /= det;
            ans += 3 * na + nb;
        }
        ans.to_string()
    }
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day13::test_input();
        let ans = Day13::solve_part_1(input);
        assert_eq!(ans, "480");
    }

    #[test]
    fn test_part_2() {
        assert!(true);
    }
}
