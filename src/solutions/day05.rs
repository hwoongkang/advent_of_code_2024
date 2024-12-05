use super::Solution;

pub struct Day05;

struct Page {
    // me: u8,
    parents: Vec<u8>,
}
const NUM_PAGE: usize = 100;
impl Page {
    fn new() -> Self {
        Self { parents: vec![] }
    }

    fn add_parent(&mut self, parent: u8) {
        self.parents.push(parent)
    }
}

fn is_valid_update(nums: &[usize], pages: &[Option<Page>]) -> bool {
    let mut visited = vec![false; NUM_PAGE];
    for num in nums.iter().rev() {
        visited[*num] = true;
        let Some(page) = &pages[*num] else {
            println!("irrelavalent");
            continue;
        };
        for parent in page.parents.iter() {
            if visited[*parent as usize] {
                return false;
            }
        }
    }
    true
}

fn sort_update(update: &mut Vec<usize>, pages: &[Option<Page>]) -> usize {
    let mut exists = vec![false; NUM_PAGE];
    for page in update.iter() {
        exists[*page] = true;
    }
    let mut sorted = vec![];
    let mut visited = vec![false; NUM_PAGE];
    fn visit(
        n: usize,
        exists: &[bool],
        visited: &mut Vec<bool>,
        sorted: &mut Vec<usize>,
        pages: &[Option<Page>],
    ) {
        if visited[n] {
            return;
        }
        let Some(page) = &pages[n] else {
            sorted.push(n);
            return;
        };
        for &parent in page.parents.iter() {
            let parent = parent as usize;
            if !exists[parent] {
                continue;
            }
            visit(parent, exists, visited, sorted, pages);
        }
        visited[n] = true;
        sorted.push(n);
    }
    for i in 0..NUM_PAGE {
        visit(i, &exists, &mut visited, &mut sorted, pages);
    }
    let mut sort_key = vec![100; NUM_PAGE];
    for (i, &n) in sorted.iter().enumerate() {
        sort_key[n] = i;
    }
    update.sort_by_key(|&n| sort_key[n]);
    update[update.len() / 2]
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
        let mut pages: Vec<Option<Page>> = (0..NUM_PAGE).map(|_| None).collect();
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

        for line in lines {
            let nums: Vec<usize> = line.split(",").map(|w| w.parse().unwrap()).collect();
            let mid = nums[nums.len() / 2];
            if is_valid_update(&nums, &pages) {
                ans += mid
            }
        }
        ans.to_string()
    }

    fn solve_part_2(_input: String) -> String {
        let mut pages: Vec<Option<Page>> = (0..NUM_PAGE).map(|_| None).collect();
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

        /***
         * L ← Empty list that will contain the sorted nodes
        while exists nodes without a permanent mark do
        select an unmarked node n
        visit(n)

        function visit(node n)
        if n has a permanent mark then
        return
        if n has a temporary mark then
        stop   (graph has at least one cycle)

        mark n with a temporary mark

        for each node m with an edge from n to m do
        visit(m)

        mark n with a permanent mark
        add n to head of L
                         */

        let mut ans = 0;
        for line in lines {
            let mut nums: Vec<usize> = line.split(",").map(|w| w.parse().unwrap()).collect();
            if !is_valid_update(&nums, &pages) {
                ans += sort_update(&mut nums, &pages);
            }
        }
        ans.to_string()
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
        assert_eq!(ans, "123");
    }
}
