use crate::Solution;

pub struct Day02 {}

fn to_index(char: char) -> usize {
    (char as u32 - 'a' as u32) as usize
}

fn checksum(line: &str) -> [usize; 26] {
    let mut appearances = [0; 26];
    for char in line.chars() {
        appearances[to_index(char)] += 1;
    }
    appearances
}

impl Solution for Day02 {
    fn test_input() -> String {
        String::from(
            "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab",
        )
    }
    fn solve_part_1(input: String) -> String {
        let (two, three) = input
            .lines()
            .map(checksum)
            .map(|checksum| {
                let two = checksum.iter().any(|a| *a == 2);
                let three = checksum.iter().any(|a| *a == 3);

                (i32::from(two), i32::from(three))
            })
            .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
        (two * three).to_string()
    }
    fn solve_part_2(input: String) -> String {
        let words: Vec<_> = input.lines().collect();
        let l = words.len();
        for i in 0..l {
            for j in (i + 1)..l {
                let word_len = words[i].len();
                let diff: String = words[i]
                    .chars()
                    .zip(words[j].chars())
                    .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
                    .collect();
                if diff.len() == word_len - 1 {
                    return diff;
                }
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod day02_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day02::test_input();
        let ans = Day02::solve_part_1(input);
        assert_eq!(ans, "12")
    }

    #[test]
    fn test_part_2() {
        let input = String::from(
            "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz",
        );
        let ans = Day02::solve_part_2(input);
        assert_eq!(ans, "fgij")
    }
}
