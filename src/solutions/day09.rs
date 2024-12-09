use std::collections::VecDeque;

use super::Solution;

enum Space {
    File(u64, usize),
    Free(u64),
}

#[derive(Debug, Copy, Clone)]
struct File {
    len: u64,
    start_pos: u64,
    file_id: u64,
}

impl File {
    fn checksum(&self) -> u64 {
        let mut ans = 0;
        for i in 0..self.len {
            ans += (self.start_pos + i) * self.file_id;
        }
        ans
    }
}

pub struct Day09;

impl Solution for Day09 {
    fn test_input() -> String {
        String::from("2333133121414131402")
    }

    fn solve_part_1(_input: String) -> String {
        let mut file_id = 0;
        let mut files: VecDeque<Space> = VecDeque::new();
        let mut is_file = true;
        for ch in _input.chars() {
            let len = ch.to_digit(10).unwrap() as u64;
            if is_file {
                files.push_back(Space::File(len, file_id));
                file_id += 1;
            } else {
                files.push_back(Space::Free(len));
            }
            is_file = !is_file;
        }
        let mut check_sum = 0;
        let mut ind = 0;

        while let Some(maybe_file) = files.pop_front() {
            match maybe_file {
                Space::File(len, file_id) => {
                    for _ in 0..len {
                        check_sum += ind * file_id;
                        ind += 1;
                    }
                }
                Space::Free(mut free_len) => {
                    while let Some(maybe_file) = files.pop_back() {
                        match maybe_file {
                            Space::Free(_) => {
                                continue;
                            }
                            Space::File(file_len, file_id) => {
                                if file_len > free_len {
                                    let diff = file_len - free_len;
                                    for _ in 0..free_len {
                                        check_sum += ind * file_id;
                                        ind += 1;
                                    }
                                    files.push_back(Space::File(diff, file_id));
                                    break;
                                } else {
                                    let diff = free_len - file_len;
                                    for _ in 0..file_len {
                                        check_sum += ind * file_id;
                                        ind += 1;
                                    }
                                    free_len = diff;
                                }
                            }
                        }
                    }
                }
            }
        }

        check_sum.to_string()
    }

    fn solve_part_2(_input: String) -> String {
        let mut file_id = 0;
        let mut files: Vec<File> = vec![];
        let mut free_spaces: VecDeque<File> = VecDeque::new();
        let mut is_file = true;
        let mut total_len = 0;
        for ch in _input.chars() {
            let len = ch.to_digit(10).unwrap() as u64;
            let file = File {
                len,
                start_pos: total_len,
                file_id,
            };
            if is_file {
                file_id += 1;
                files.push(file);
            } else {
                free_spaces.push_back(file);
            }
            total_len += len;
            is_file = !is_file;
        }

        let mut check_sum = 0;
        while let Some(mut file) = files.pop() {
            for i in 0..free_spaces.len() {
                let free_space = &mut free_spaces[i];

                // nowhere to fit, surpassed the file itself.
                if free_space.start_pos > file.start_pos {
                    check_sum += file.checksum();
                    break;
                }
                // this space is to narrow
                else if free_space.len < file.len {
                    continue;
                }
                // can move the file!
                else {
                    file.start_pos = free_space.start_pos;
                    check_sum += file.checksum();
                    free_space.start_pos += file.len;
                    free_space.len -= file.len;
                    break;
                }
            }
        }

        check_sum.to_string()
    }
}

impl Day09 {
    pub fn solve_part_1_naive(input: String) -> String {
        let mut files: VecDeque<Option<u64>> = VecDeque::new();
        let mut is_file = true;
        let mut file_id = 0;
        for ch in input.chars() {
            let len = ch.to_digit(10).unwrap();
            if is_file {
                for _ in 0..len {
                    files.push_back(Some(file_id));
                }
                is_file = false;
                file_id += 1;
            } else {
                for _ in 0..len {
                    files.push_back(None);
                }
                is_file = true;
            }
        }

        let mut check_sum = 0;
        let mut ind = 0;
        while let Some(maybe_file) = files.pop_front() {
            if let Some(file_id) = maybe_file {
                check_sum += file_id * ind;
            } else {
                while let Some(maybe_file) = files.pop_back() {
                    if let Some(file_id) = maybe_file {
                        check_sum += file_id * ind;
                        break;
                    }
                }
            }
            ind += 1;
        }

        check_sum.to_string()
    }
}

#[cfg(test)]
mod day09_tests {
    use super::*;

    #[test]
    fn test_part_1_naive() {
        let input = Day09::test_input();
        let ans = Day09::solve_part_1_naive(input);
        assert_eq!(ans, "1928");
    }

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
        assert_eq!(ans, "2858");
    }
}
