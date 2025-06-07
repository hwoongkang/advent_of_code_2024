use crate::Solution;
const BASE_SIGNAL: [i32; 4] = [0, 1, 0, -1];

fn repeated_base(i: usize) -> impl Iterator<Item = i32> {
    BASE_SIGNAL
        .iter()
        .cycle()
        .map(move |num| vec![*num; i])
        .flatten()
        .skip(1)
}

fn fft(signal: String) -> String {
    let l = signal.len();

    let nums: Vec<i32> = signal
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as i32)
        .collect();

    (0..l)
        .map(|i| repeated_base(i + 1))
        .map(|it| {
            nums.iter()
                .zip(it.take(l))
                .map(|(n, m)| n * m)
                .sum::<i32>()
                .abs()
                % 10
        })
        .map(|n| char::from_digit(n as u32, 10).unwrap())
        .collect()
}

pub struct Day16 {}

impl Solution for Day16 {
    fn test_input() -> String {
        String::from("80871224585914546619083218645595")
    }
    fn solve_part_1(input: String) -> String {
        let mut input = input;

        for _ in 0..100 {
            input = fft(input);
        }

        input[..8].to_string()
    }
    fn solve_part_2(input: String) -> String {
        let l = input.len();

        // A B C  x1    y1
        // D E F  x2  = y2
        // G H I  x3    y3
        // WHERE D, G, H = 0
        // And F is all ones,
        // E and I are upper triangle ones
        // y1 = Ax1 + Bx2 + Cx3
        // y2 =       Ex2 + Fx3
        // y3 =           + Ix3
        let skip: usize = input[..7].parse().unwrap();

        let repeated = input.chars().cycle().take(l * 10_000).skip(skip);
        let mut x2: Vec<u32> = repeated.take(8).map(|c| c.to_digit(10).unwrap()).collect();
        let repeated = input.chars().cycle().take(l * 10_000).skip(skip);
        let mut x3: Vec<u32> = repeated.skip(8).map(|c| c.to_digit(10).unwrap()).collect();

        for _ in 0..100 {
            fn sum(x: &[u32], start: usize) -> u32 {
                x.iter().skip(start).sum()
            }
            let x3_sum = sum(&x3, 0);
            let mut new_x3 = vec![];
            let mut acc = 0;
            for x in x3.iter().rev() {
                acc += x;
                new_x3.push(acc % 10);
            }
            x2 = (0..8)
                .into_iter()
                .map(|i| x3_sum + sum(&x2, i))
                .map(|i| i % 10)
                .collect();
            x3 = new_x3.into_iter().rev().collect();
        }

        x2.into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    #[test]
    fn test_signal() {
        assert_eq!(vec![1, 0, -1], repeated_base(1).take(3).collect::<Vec<_>>());
        assert_eq!(
            vec![0, 1, 1, 0, 0, -1, -1],
            repeated_base(2).take(7).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1],
            repeated_base(3).take(11).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_part_1() {
        let input = Day16::test_input();
        let ans = Day16::solve_part_1(input);
        assert_eq!(ans, "24176176")
    }

    #[test]
    fn test_part_2() {
        let input = Day16::test_input();
        let ans = Day16::solve_part_2(input);
        assert_eq!(ans, "0")
    }
}
