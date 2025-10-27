use crate::Solution;

const MAGIC_NUMBER: usize = 20201227;

const SUBJECT_NUMBER: usize = 7;

fn get_public_key(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for i in 0..loop_size {
        value *= subject_number;
        value %= MAGIC_NUMBER;
    }
    value
}

fn find_loop_size(public_key: usize) -> usize {
    let mut value = 1;
    for i in 0.. {
        if value == public_key {
            return i;
        }
        value *= SUBJECT_NUMBER;
        value %= MAGIC_NUMBER;
    }
    0
}

pub struct Day25 {}

impl Solution for Day25 {
    fn test_input() -> String {
        String::from(
            "5764801
17807724",
        )
    }
    fn solve_part_1(input: String) -> String {
        let mut lines = input.lines();
        let pdoor = lines.next().unwrap().parse().unwrap();
        let ldoor = find_loop_size(pdoor);
        let pkey = lines.next().unwrap().parse().unwrap();
        let lkey = find_loop_size(pkey);

        let (number, size) = if ldoor > lkey {
            (pdoor, lkey)
        } else {
            (pkey, ldoor)
        };

        get_public_key(number, size).to_string()
    }
    fn solve_part_2(input: String) -> String {
        String::new()
    }
}

#[cfg(test)]
mod day25_tests {
    use super::*;
    #[test]
    fn test_example() {
        let public_key = 5764801;
        assert_eq!(find_loop_size(public_key), 8);
        let public_key = 17807724;
        assert_eq!(find_loop_size(public_key), 11)
    }

    #[test]
    fn test_part_1() {
        let input = Day25::test_input();
        let ans = Day25::solve_part_1(input);
        assert_eq!(ans, "14897079")
    }

    #[test]
    fn test_part_2() {
        let input = Day25::test_input();
        let ans = Day25::solve_part_2(input);
        assert_eq!(ans, "0")
    }
}
