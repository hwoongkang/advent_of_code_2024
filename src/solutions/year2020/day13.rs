use crate::Solution;

pub struct Day13 {}

impl Solution for Day13 {
    fn test_input() -> String {
        String::from(
            "939
7,13,x,x,59,x,31,19",
        )
    }
    fn solve_part_1(input: String) -> String {
        let lines = &mut input.lines();
        let t: u32 = lines.next().unwrap().parse().unwrap();
        let t = t - 1;
        let mut min = u32::MAX;
        let mut ans = 0;
        for bus in lines.next().unwrap().split(",") {
            let Ok(bus) = bus.parse::<u32>() else {
                continue;
            };
            let rem = t % bus;

            let waittime = bus - rem - 1;

            if waittime < min {
                min = waittime;
                ans = waittime * bus;
            }
        }
        ans.to_string()
    }
    fn solve_part_2(input: String) -> String {
        let buses: Vec<(u64, u64)> = input
            .lines()
            .nth(1)
            .unwrap()
            .split(",")
            .enumerate()
            .filter_map(|(ind, word)| {
                if let Ok(bus) = word.parse() {
                    Some((ind as u64, bus))
                } else {
                    None
                }
            })
            .collect();
        let (_, mut chunk) = buses[0];
        let mut t = chunk;

        for (ind, bus) in buses[1..].iter() {
            let desired_rem = bus - (ind % bus);

            while t % bus != desired_rem {
                t += chunk;
            }
            chunk *= bus;
        }

        t.to_string()
    }
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day13::test_input();
        let ans = Day13::solve_part_1(input);
        assert_eq!(ans, "295")
    }

    #[test]
    fn test_part_2() {
        let input = Day13::test_input();
        let ans = Day13::solve_part_2(input);
        assert_eq!(ans, "1068781")
    }

    #[test]
    fn test_additionals() {
        let input = String::from(
            "
17,x,13,19",
        );
        let ans = 3417;
        assert_eq!(Day13::solve_part_2(input), ans.to_string());
        let input = String::from(
            "
67,7,59,61",
        );
        let ans = 754018;
        assert_eq!(Day13::solve_part_2(input), ans.to_string());
        let input = String::from(
            "
67,x,7,59,61",
        );
        let ans = 779210;
        assert_eq!(Day13::solve_part_2(input), ans.to_string());
        let input = String::from(
            "
67,7,x,59,61",
        );
        let ans = 1261476;
        assert_eq!(Day13::solve_part_2(input), ans.to_string());
        let input = String::from(
            "
1789,37,47,1889",
        );
        let ans = 1202161486;
        assert_eq!(Day13::solve_part_2(input), ans.to_string());
    }
}
