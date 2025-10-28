use crate::Solution;

#[derive(PartialEq, Eq, Debug)]
enum Shuffle {
    NewStack,
    Cut(i128),
    Increment(i128),
}

// let xth card number be f(x)
// and the number of cards in the whole set be N
// what happens for each shuffle?
// 1. NewStack
// let the new map be g(x)
// g(x) = f(N-x)
// or maybe we can define all the x by modulo N
// and say g(x) = f(-x-1)

// 2. Cut t
// g(x) = f(x+t)

// 3. Increment t
// it's more like g(tx) = f(x)
// thus we need modulo inverse...?
// fermat's little theorem -> a^(N-2) = a^(-1) (if N is a prime)
// thus
// g(x) = f(t^(N-2) x)

// since f(x) starts from f(x) = x
// and all the mappings are linear (very loose idea)
// lets just map (a, b) of f(x) = ax + b

// 1. NewStack
// a_post, b_post = -a_pre, b_pre
// 2. Cut t
// a_post, b_post = a_pre, a_pre * t + b_pre
// 3. Increment t
// a_post, b_post = a_pre * t ^ (N-2), b_pre
// number theory applited

fn fast_power(mut base: i128, mut exponent: i128, modulo: i128) -> i128 {
    let mut ans = 1;
    while exponent > 0 {
        if exponent % 2 == 1 {
            ans *= base;
            ans %= modulo;
        }
        base *= base;
        base %= modulo;
        exponent /= 2;
    }
    ans
}
impl Shuffle {
    fn transform(&self, pre: (i128, i128), size: i128) -> (i128, i128) {
        let (a0, b0) = pre;

        let (a1, b1) = match self {
            Self::NewStack => (-a0, -a0 + b0),
            Self::Cut(t) => (a0, a0 * t + b0),
            Self::Increment(t) => {
                let inverse = fast_power(*t, size - 2, size);
                (a0 * inverse, b0)
            }
        };
        (a1.rem_euclid(size), b1.rem_euclid(size))
    }
}

impl Shuffle {
    fn from(line: &str) -> Self {
        if line.starts_with("deal") {
            if line.ends_with("stack") {
                Self::NewStack
            } else {
                let num = line
                    .split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();
                Self::Increment(num)
            }
        } else {
            let num = line
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            Self::Cut(num)
        }
    }
}

pub struct Day22 {}

impl Solution for Day22 {
    fn test_input() -> String {
        String::from(
            "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1",
        )
    }
    fn solve_part_1(input: String) -> String {
        let size = 10_007;
        let mut transform = (1, 0);

        for line in input.lines() {
            let cmd = Shuffle::from(line);
            transform = cmd.transform(transform, size);
        }
        let (a, b) = transform;
        println!("(a, b) = ({}, {})", a, b);
        for x in 0..size {
            let f = (a * x + b) % size;
            if f == 2019 {
                return x.to_string();
            }
        }
        String::new()
    }
    fn solve_part_2(input: String) -> String {
        let cards: i128 = 119315717514047;
        let iters: i128 = 101_741_582_076_661;
        let size = cards;
        let mut transform = (1, 0);

        for line in input.lines() {
            let cmd = Shuffle::from(line);
            transform = cmd.transform(transform, size);
        }

        let (a, b) = transform;
        println!("(a, b) = ({}, {})", a, b);
        // final a, b would be
        // a ^ (iters), b * (a^(iters - 1) + ... + 1)
        // or
        // a ^ (iters), b * (a^iters - 1) / (a - 1)

        let final_a = fast_power(a, iters, cards);
        let inverse = fast_power(a - 1, cards - 2, cards);
        let mut final_b = b;
        final_b *= final_a - 1;
        final_b %= cards;
        final_b *= inverse;
        final_b %= cards;

        let mut ans = 2020 * final_a + final_b;
        ans %= cards;
        ans.to_string()
    }
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    #[test]
    fn test_power() {
        assert_eq!(
            fast_power(2, 61, 101741582076661),
            2i128.pow(61) % 101741582076661
        )
    }

    #[test]
    fn test_parse() {
        let input = String::from(
            "cut 6
deal with increment 7
deal into new stack
cut -2",
        );
        let cmds: Vec<Shuffle> = input.lines().map(Shuffle::from).collect();
        assert_eq!(
            cmds,
            vec![
                Shuffle::Cut(6),
                Shuffle::Increment(7),
                Shuffle::NewStack,
                Shuffle::Cut(-2)
            ]
        )
    }

    #[test]
    fn test_number_theory() {
        let pre = (1, 0);
        let (a, b) = Shuffle::NewStack.transform(pre, 11);
        assert_eq!(
            (0..11)
                .map(|i| (a * i) + b)
                .map(|i| i % 11)
                .collect::<Vec<_>>(),
            vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
        );
        let (a, b) = Shuffle::Cut(3).transform(pre, 11);
        assert_eq!(
            (0..11)
                .map(|i| (a * i) + b)
                .map(|i| i % 11)
                .collect::<Vec<_>>(),
            vec![3, 4, 5, 6, 7, 8, 9, 10, 0, 1, 2]
        );
        let (a, b) = Shuffle::Cut(-4).transform(pre, 11);
        assert_eq!(
            (0..11)
                .map(|i| (a * i) + b)
                .map(|i| i % 11)
                .collect::<Vec<_>>(),
            vec![7, 8, 9, 10, 0, 1, 2, 3, 4, 5, 6]
        );
        let (a, b) = Shuffle::Increment(3).transform(pre, 11);
        assert_eq!(
            (0..11)
                .map(|i| (a * i) + b)
                .map(|i| i % 11)
                .collect::<Vec<_>>(),
            vec![0, 4, 8, 1, 5, 9, 2, 6, 10, 3, 7]
        )
    }

    #[test]
    fn test_part_1() {
        let input = Day22::test_input();
        let ans = Day22::solve_part_1(input);
        assert_eq!(ans, "0")
    }

    #[test]
    fn test_part_2() {
        let input = Day22::test_input();
        let ans = Day22::solve_part_2(input);
        assert_eq!(ans, "0")
    }
}
