use std::collections::HashMap;

use crate::Solution;

type Cache = HashMap<(u8, u8, bool), i32>;

fn test(mut n: i32) -> bool {
    let mut nums = vec![];
    while n > 0 {
        nums.push(n % 10);
        n /= 10;
    }
    for i in 0..5 {
        if nums[i] < nums[i + 1] {
            return false;
        }
    }

    let mut i = 0;
    while i < 6 {
        let mut j = i;
        // 111122
        // i = 0, j = 0 -> 1, 1
        // i = 0, j = 1 -> 1, 1
        // i = 0, j = 2 -> 1, 1
        // i = 0, j = 3 -> 1, 1
        // i = 0, j = 4 -> 1, 2
        // amount = 4
        // flag = false
        // i = 4, j = 4 -> 2, 2
        // i = 4, j = 5 -> 2, 2
        // i = 4, j = 6
        // amount = 2
        // flag = true
        // i = 6
        // break

        while j < 6 && nums[i] == nums[j] {
            j += 1;
        }
        let amount = j - i;
        if amount == 2 {
            return true;
        }

        i = j;
    }

    false
}

// 34577* -> start = 7, remaining = 1, criteria_met = true
fn helper(start: u8, remaining: u8, criteria_met: bool, cache: &mut Cache) -> i32 {
    if remaining == 0 {
        if criteria_met {
            return 1;
        } else {
            return 0;
        }
    }
    let key = (start, remaining, criteria_met);
    if let Some(ans) = cache.get(&key) {
        return *ans;
    }
    let mut ans = 0;
    for n in start..=9 {
        let new_criteria_met = if n == start { true } else { criteria_met };
        ans += helper(n, remaining - 1, new_criteria_met, cache);
    }
    cache.insert(key, ans);
    ans
}

pub struct Day04 {}

impl Solution for Day04 {
    fn test_input() -> String {
        String::from("171309-643603")
    }
    fn solve_part_1(_input: String) -> String {
        let mut cache: Cache = HashMap::new();
        let mut ans = 0;

        // 17**** -> start = 7, remaining = 4, criteria_met = false
        // fn helper(start: u8, remaining: u8, criteria_met: bool, cache: &mut Cache) -> i32 {
        ans += helper(7, 4, false, &mut cache);
        ans += helper(8, 4, false, &mut cache);
        ans += helper(9, 4, false, &mut cache);
        ans += helper(2, 5, false, &mut cache);
        ans += helper(3, 5, false, &mut cache);
        ans += helper(4, 5, false, &mut cache);
        ans += helper(5, 5, false, &mut cache);

        ans.to_string()
    }
    fn solve_part_2(_input: String) -> String {
        let mut ans = 0;
        for i in 171309..643603 {
            if test(i) {
                ans += 1;
            }
        }
        ans.to_string()
    }
}

#[cfg(test)]
mod day04_tests {
    use super::*;

    #[test]
    fn test_part_2_basics() {
        assert!(test(112233));
        assert!(!test(123444));
        assert!(test(111122));
    }

    #[test]
    fn test_part_1() {
        let input = Day04::test_input();
        let ans = Day04::solve_part_1(input);
        assert_eq!(ans, "")
    }

    #[test]
    fn test_part_2() {
        let input = Day04::test_input();
        let ans = Day04::solve_part_2(input);
        assert_eq!(ans, "0")
    }
}
