use std::collections::HashMap;

use crate::Solution;

pub struct Day04 {}

#[derive(PartialEq, Eq, Debug)]
struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}-{:0>2}-{:0>2} {:0>2}:{:0>2}]",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.year.partial_cmp(&other.year) {
            Some(std::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.month.partial_cmp(&other.month) {
            Some(std::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.day.partial_cmp(&other.day) {
            Some(std::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.hour.partial_cmp(&other.hour) {
            Some(std::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minute.partial_cmp(&other.minute)
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl DateTime {
    fn from(s: &str) -> Self {
        let year = s[1..5].parse().unwrap();
        let month = s[6..8].parse().unwrap();
        let day = s[9..11].parse().unwrap();
        let hour = s[12..14].parse().unwrap();
        let minute = s[15..17].parse().unwrap();
        Self {
            year,
            month,
            day,
            hour,
            minute,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum LogContent {
    FallsAsleep,
    WakesUp,
    Begins(i32),
}

impl std::fmt::Display for LogContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FallsAsleep => {
                write!(f, "falls asleep")
            }
            Self::WakesUp => {
                write!(f, "wakes up")
            }
            Self::Begins(id) => {
                write!(f, "Guard #{} begins shift", id)
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Log {
    datetime: DateTime,
    content: LogContent,
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.datetime, self.content)
    }
}

impl PartialOrd for Log {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.datetime.partial_cmp(&other.datetime)
    }
}

impl Ord for Log {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Log {
    fn from(s: &str) -> Self {
        Self {
            datetime: DateTime::from(s),
            content: LogContent::from(s),
        }
    }
}

impl LogContent {
    fn from(s: &str) -> Self {
        let s = &s[19..];
        if s.starts_with("falls") {
            Self::FallsAsleep
        } else if s.starts_with("wakes") {
            Self::WakesUp
        } else {
            let id = (&s.split_ascii_whitespace().nth(1).unwrap())[1..]
                .parse()
                .unwrap();

            Self::Begins(id)
        }
    }
}

impl Solution for Day04 {
    fn test_input() -> String {
        String::from(
            "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #991 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #991 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #991 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up",
        )
    }
    fn solve_part_1(input: String) -> String {
        let mut guard_id = -1;
        let mut sleep_started = 0;
        let mut sleep_logs: HashMap<i32, ([u8; 60], i32)> = HashMap::new();

        let mut logs: Vec<_> = input.lines().map(Log::from).collect();
        logs.sort();

        for log in logs {
            match log.content {
                LogContent::Begins(id) => guard_id = id,
                LogContent::FallsAsleep => sleep_started = log.datetime.minute,
                LogContent::WakesUp => {
                    let v = sleep_logs.entry(guard_id).or_insert(([0; 60], 0));
                    let sleep_ended = log.datetime.minute;
                    let delta = sleep_ended - sleep_started;
                    v.1 += delta as i32;
                    for m in sleep_started..log.datetime.minute {
                        let m = m as usize;
                        v.0[m] += 1;
                    }
                }
            }
        }

        let max = sleep_logs
            .into_iter()
            .map(|(guard_id, (log, total))| {
                let max_minute = log.iter().enumerate().max_by_key(|tup| tup.1).unwrap().0;
                (guard_id, max_minute as i32, total)
            })
            .max_by_key(|tup| tup.2)
            .unwrap();

        (max.0 * max.1).to_string()
    }
    fn solve_part_2(input: String) -> String {
        let mut guard_id = -1;
        let mut sleep_started = 0;
        let mut sleep_logs: HashMap<i32, [u8; 60]> = HashMap::new();

        let mut logs: Vec<_> = input.lines().map(Log::from).collect();
        logs.sort();

        for log in logs {
            println!("{}", log);
            match log.content {
                LogContent::Begins(id) => guard_id = id,
                LogContent::FallsAsleep => {
                    if log.datetime.hour != 0 {
                        panic!("Problem detected")
                    }
                    sleep_started = log.datetime.minute
                }
                LogContent::WakesUp => {
                    if log.datetime.hour != 0 {
                        panic!("Problem detected")
                    }
                    let v = sleep_logs.entry(guard_id).or_insert([0; 60]);
                    let sleep_ended = log.datetime.minute;

                    for m in sleep_started..sleep_ended {
                        let m = m as usize;
                        v[m] += 1;
                    }
                }
            }
        }

        for log in sleep_logs.iter() {
            println!("Guard #{}", log.0);
            print!("\t");
            for freq in log.1.iter() {
                print!("{} ", freq)
            }
            println!();
            let max = log.1.iter().enumerate().max_by_key(|tup| tup.1).unwrap();
            println!("\tmax: {:?}", max);
        }

        let max = sleep_logs
            .into_iter()
            .map(|(guard_id, log)| {
                let max = log.into_iter().enumerate().max_by_key(|tup| tup.1).unwrap();
                let (minute, freq) = max;
                (guard_id, minute as i32, freq)
            })
            .max_by_key(|tup| tup.2)
            .unwrap();
        (max.0 * max.1).to_string()
    }
}

#[cfg(test)]
mod day04_tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = String::from(
            "[1518-11-01 23:58] Guard #991 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up",
        );
        let logs: Vec<_> = input.lines().map(Log::from).collect();
        assert_eq!(
            logs,
            vec![
                Log {
                    datetime: DateTime {
                        year: 1518,
                        month: 11,
                        day: 1,
                        hour: 23,
                        minute: 58
                    },
                    content: LogContent::Begins(991)
                },
                Log {
                    datetime: DateTime {
                        year: 1518,
                        month: 11,
                        day: 2,
                        hour: 0,
                        minute: 40
                    },
                    content: LogContent::FallsAsleep
                },
                Log {
                    datetime: DateTime {
                        year: 1518,
                        month: 11,
                        day: 2,
                        hour: 0,
                        minute: 50
                    },
                    content: LogContent::WakesUp
                }
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let input = Day04::test_input();
        let ans = Day04::solve_part_1(input);
        assert_eq!(ans, "240")
    }

    #[test]
    fn test_part_2() {
        let input = Day04::test_input();
        let ans = Day04::solve_part_2(input);
        assert_eq!(ans, (991 * 45).to_string())
    }
}
