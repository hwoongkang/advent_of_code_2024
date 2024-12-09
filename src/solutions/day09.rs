use std::collections::VecDeque;

use super::Solution;

enum Space {
    File(usize, usize),
    Free(usize),
}

pub struct Day09;

impl Solution for Day09 {
    fn test_input() -> String {
        String::from("2333133121414131402")
    }

    fn solve_part_1(_input: String) -> String {
        let mut file_id = 0;
        let mut files: VecDeque<Option<usize>> = VecDeque::new();
        let mut is_file = true;
        for char in _input.chars() {
            let num = char.to_digit(10).unwrap();
            let file = if !is_file { None } else { Some(file_id) };
            for _ in 0..num {
                files.push_back(file);
            }
            is_file = !is_file;
            if is_file {
                file_id += 1;
            }
        }
        let mut check_sum = 0;
        let mut ind = 0;
        while let Some(maybe_file) = files.pop_front() {
            if let Some(file_id) = maybe_file {
                check_sum += ind * file_id;
            } else {
                loop {
                    if let Some(maybe_file) = files.pop_back() {
                        if let Some(file_id) = maybe_file {
                            check_sum += ind * file_id;
                            break;
                        }
                    }
                }
            }
            ind += 1;
        }
        check_sum.to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod day09_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day09::test_input();
        let ans = Day09::solve_part_1(input);
        assert_eq!(ans, "1928");
    }

    #[test]
    fn test_part_2() {
        let input = Day09::test_input();
        let ans = Day09::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
