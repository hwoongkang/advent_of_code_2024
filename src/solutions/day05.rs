use super::Solution;

pub struct Day05;

struct Page {
    // me: u8,
    parents: Vec<u8>,
}

impl Page {
    fn new() -> Self {
        Self { parents: vec![] }
    }

    fn add_parent(&mut self, parent: u8) {
        self.parents.push(parent)
    }
}

impl Solution for Day05 {
    fn test_input() -> String {
        String::from(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        )
    }

    fn solve_part_1(_input: String) -> String {
        let mut pages: Vec<Option<Page>> = (0..100).map(|_| None).collect();
        let mut lines = _input.lines();

        for line in &mut lines {
            if line == "" {
                break;
            }
            let mut words = line.split("|");
            let parent: u8 = words.next().unwrap().parse().unwrap();
            let child: u8 = words.next().unwrap().parse().unwrap();

            if pages[child as usize].is_none() {
                pages[child as usize] = Some(Page::new());
            }
            if pages[parent as usize].is_none() {
                let new_parent = Page::new();
                pages[parent as usize] = Some(new_parent);
            }

            if let Some(page) = &mut pages[child as usize] {
                page.add_parent(parent);
            } else {
                let mut new_page = Page::new();
                new_page.add_parent(parent);
            }
        }

        let mut ans = 0;

        'outmost: for line in lines {
            let nums: Vec<usize> = line.split(",").map(|w| w.parse().unwrap()).collect();
            let mid = nums[nums.len() / 2];
            let mut visited = vec![false; 100];
            for num in nums.iter().rev() {
                visited[*num] = true;
                let Some(page) = &pages[*num] else {
                    continue;
                };
                for parent in page.parents.iter() {
                    if visited[*parent as usize] {
                        continue 'outmost;
                    }
                }
            }
            ans += mid
        }
        ans.to_string()
    }

    fn solve_part_2(_input: String) -> String {
        String::from("0")
    }
}

#[cfg(test)]
mod day05_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day05::test_input();
        let ans = Day05::solve_part_1(input);
        assert_eq!(ans, "143");
    }

    #[test]
    fn test_part_2() {
        let input = Day05::test_input();
        let ans = Day05::solve_part_2(input);
        assert_eq!(ans, "");
    }
}
