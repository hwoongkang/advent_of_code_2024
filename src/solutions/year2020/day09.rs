use crate::Solution;

fn part1(input: String, preamble: usize) -> usize {
    let lines = &mut input.lines();
    let mut queue: Vec<usize> = vec![];
    for line in lines.take(preamble) {
        queue.push(line.parse().unwrap());
    }
    'outer: for line in lines {
        let num: usize = line.parse().unwrap();
        let l = queue.len();
        queue.push(num);
        for i in (l - preamble)..l {
            for j in i + 1..l {
                let sum = queue[i] + queue[j];
                if sum == num {
                    continue 'outer;
                }
            }
        }
        return num;
    }
    0
}

fn part2(input: String, target: usize) -> usize {
    let nums: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut sum = 0;
    let mut sums = vec![0];
    for num in nums.iter() {
        sum += num;
        sums.push(sum);
    }
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if sums[j] - sums[i] == target {
                let interval = &nums[i..j];
                let min = interval.iter().min().unwrap();
                let max = interval.iter().max().unwrap();
                return min + max;
            }
        }
    }
    0
}

pub struct Day09 {}

impl Solution for Day09 {
    fn test_input() -> String {
        String::from(
            "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576",
        )
    }
    fn solve_part_1(input: String) -> String {
        part1(input, 25).to_string()
    }
    fn solve_part_2(input: String) -> String {
        part2(input, 26134589).to_string()
    }
}

#[cfg(test)]
mod day09_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day09::test_input();
        let ans = part1(input, 5);
        assert_eq!(ans, 127)
    }

    #[test]
    fn test_part_2() {
        let input = Day09::test_input();
        let ans = part2(input, 127);
        assert_eq!(ans, 62)
    }
}
