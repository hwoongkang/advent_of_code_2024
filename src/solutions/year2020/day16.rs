use crate::Solution;

struct Range(i64, i64);

impl Range {
    fn from(word: &str) -> Self {
        let mut divided = word.split("-");
        let from: i64 = divided.next().unwrap().parse().unwrap();
        let to: i64 = divided.next().unwrap().parse().unwrap();
        Self(from, to + 1)
    }

    fn contains(&self, num: i64) -> bool {
        self.0 <= num && num < self.1
    }
}

struct Field {
    _name: String,
    ranges: Vec<Range>,
}

impl Field {
    fn from(line: &str) -> Self {
        let mut parts = line.split(": ");
        let name = parts.next().unwrap().to_string();
        let words = parts.next().unwrap().split(" or ");
        Self {
            _name: name,
            ranges: words.map(Range::from).collect(),
        }
    }

    fn contains(&self, num: i64) -> bool {
        self.ranges.iter().any(|range| range.contains(num))
    }
}

pub struct Day16 {}

impl Solution for Day16 {
    fn test_input() -> String {
        String::from(
            "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12",
        )
    }
    fn solve_part_1(input: String) -> String {
        let mut fields = vec![];
        let mut lines = input.lines();
        for line in &mut lines {
            if line == "" {
                break;
            }
            fields.push(Field::from(line));
        }
        let mut ans = 0;
        for line in lines.skip(4) {
            for num in line.split(",").map(|w| w.parse().unwrap()) {
                if !fields.iter().any(|field| field.contains(num)) {
                    ans += num;
                }
            }
        }

        ans.to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut fields = vec![];
        let mut lines = input.lines();
        for line in &mut lines {
            if line == "" {
                break;
            }
            fields.push(Field::from(line));
        }

        let my_ticket: Vec<i64> = lines
            .nth(1)
            .unwrap()
            .split(",")
            .map(|w| w.parse().unwrap())
            .collect();

        let mut tickets: Vec<Vec<i64>> = lines
            .skip(2)
            .map(|line| line.split(",").map(|w| w.parse().unwrap()).collect())
            .collect();

        tickets = tickets
            .into_iter()
            .filter_map(|ticket| {
                let is_valid = ticket
                    .iter()
                    .all(|&num| fields.iter().any(|field| field.contains(num)));
                if is_valid {
                    Some(ticket)
                } else {
                    None
                }
            })
            .collect();

        let mut possible_fields: Vec<Vec<i32>> = vec![];

        for ticket in tickets.iter() {
            let mut shuffled = vec![];
            for num in ticket.iter() {
                let mut flags = 0;
                for (ind, field) in fields.iter().enumerate() {
                    if field.contains(*num) {
                        flags |= 1 << ind;
                    }
                }
                shuffled.push(flags);
            }
            possible_fields.push(shuffled);
        }
        let mut collapsed = vec![];

        for i in 0..20 {
            let mut flag = 0;
            for j in 0..20 {
                flag |= 1 << j;
            }
            for ticket in possible_fields.iter() {
                flag &= ticket[i];
            }
            collapsed.push(flag);
        }

        let mut with_index = collapsed.into_iter().enumerate().collect::<Vec<_>>();

        with_index.sort_by_key(|t| t.1.count_ones());

        let mut occupied = 0;

        let mut ans = 1;

        for (from, possibility) in with_index.iter() {
            let flag = occupied ^ possibility;
            let to = flag.trailing_zeros();
            occupied = *possibility;
            if to < 6 {
                ans *= my_ticket[*from];
            }
        }

        ans.to_string()
    }
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day16::test_input();
        let ans = Day16::solve_part_1(input);
        assert_eq!(ans, "71")
    }

    #[test]
    fn test_part_2() {
        let input = Day16::test_input();
        let ans = Day16::solve_part_2(input);
        assert_eq!(ans, "0")
    }
}
