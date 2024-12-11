use super::Solution;

fn num_digits(mut n: usize) -> usize {
    let mut ans = 0;
    while n > 0 {
        n /= 10;
        ans += 1;
    }
    ans
}

#[derive(PartialEq, Eq, Debug)]
struct PlutoStone(usize);

impl PlutoStone {
    fn blink(&self) -> Vec<Self> {
        let num = self.0;
        match num {
            0 => vec![Self(1)],
            n => {
                let i = num_digits(n);
                if i % 2 == 0 {
                    let i = i / 2;
                    let base = 10usize.pow(i as u32);

                    vec![Self(n / base), Self(n % base)]
                } else {
                    vec![Self(n * 2024)]
                }
            }
        }
    }
}

fn blink(stones: Vec<PlutoStone>) -> Vec<PlutoStone> {
    let mut ans = vec![];
    for stone in stones.iter() {
        ans.append(&mut stone.blink());
    }
    ans
}

pub struct Day11;

impl Solution for Day11 {
    fn test_input() -> String {
        String::from("125 17")
    }

    fn solve_part_1(_input: String) -> String {
        let mut stones = _input
            .split_ascii_whitespace()
            .map(|w| w.parse().unwrap())
            .map(|n| PlutoStone(n))
            .collect();
        for _ in 0..25 {
            stones = blink(stones);
        }
        stones.len().to_string()
    }

    fn solve_part_2(_input: String) -> String {
        let mut stones = _input
            .split_ascii_whitespace()
            .map(|w| w.parse().unwrap())
            .map(|n| PlutoStone(n))
            .collect();
        for _ in 0..40 {
            stones = blink(stones);
        }
        stones.len().to_string()
    }
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn test_blink() {
        let prev = vec![
            PlutoStone(0),
            PlutoStone(1),
            PlutoStone(10),
            PlutoStone(99),
            PlutoStone(999),
        ];
        let next = vec![
            PlutoStone(1),
            PlutoStone(2024),
            PlutoStone(1),
            PlutoStone(0),
            PlutoStone(9),
            PlutoStone(9),
            PlutoStone(2021976),
        ];
        assert_eq!(blink(prev), next);
    }

    #[test]
    fn test_part_1() {
        let input = Day11::test_input();
        let ans = Day11::solve_part_1(input);
        assert_eq!(ans, "55312");
    }

    #[test]
    fn test_part_2() {
        let input = Day11::test_input();
        let ans = Day11::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
