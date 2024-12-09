use std::collections::VecDeque;

use super::Solution;

enum Space {
    File(u32, usize),
    Free(u32),
}

#[derive(Debug)]
enum Space2 {
    File(u32, u32, usize),
    Free(u32),
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
            let len = ch.to_digit(10).unwrap();
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
        let mut files: VecDeque<Space2> = VecDeque::new();
        let mut is_file = true;
        let mut total_len = 0;
        for ch in _input.chars() {
            let len = ch.to_digit(10).unwrap();
            if is_file {
                files.push_back(Space2::File(len, total_len, file_id));
                file_id += 1;
            } else {
                files.push_back(Space2::Free(len));
            }
            total_len += len;
            is_file = !is_file;
        }
        for f in files.iter() {
            println!("{:?}", f);
        }
        let mut check_sum = 0;
        let mut ind = 0;
        while let Some(maybe_file) = files.pop_front() {
            match maybe_file {
                Space2::File(file_len, start_pos, file_id) => {
                    let mut diff = 0;
                    for i in 0..file_len {
                        diff += (start_pos + i) as usize * file_id;
                    }
                    check_sum += diff;
                    println!("added... {} {}", file_id, diff);
                    ind += file_len;
                }
                Space2::Free(free_len) => {
                    while let Some(maybe_file) = files.pop_back() {
                        match maybe_file {
                            Space2::Free(_) => {
                                continue;
                            }
                            Space2::File(file_len, start_pos, file_id) => {
                                if file_len > free_len {
                                    let mut diff = 0;
                                    for i in 0..file_len {
                                        diff += (start_pos + i) as usize * file_id;
                                    }
                                    check_sum += diff;
                                    println!("added... {} {} (too long)", file_id, diff);
                                    continue;
                                } else {
                                    let fin_ind = ind + free_len;
                                    let mut diff = 0;
                                    for i in 0..file_len {
                                        diff += (ind + i) as usize * file_id;
                                    }
                                    check_sum += diff;
                                    println!("added... {} {} (fitted)", file_id, diff);
                                    ind = fin_ind;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        check_sum.to_string()
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
        assert_eq!(ans, "2858");
    }
}
