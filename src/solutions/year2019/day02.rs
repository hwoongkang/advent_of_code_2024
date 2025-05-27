use crate::Solution;

pub struct Day02 {}

impl Day02 {
    fn parse(input: String) -> Vec<usize> {
        let states: Vec<usize> = input.split(",").map(|w| w.parse().unwrap()).collect();
        states
    }
    fn simulate(mut states: Vec<usize>) -> Result<usize, ()> {
        let mut index = 0;
        loop {
            let op_code = states[index];
            if op_code != 99 && index + 3 >= states.len() {
                return Err(());
            }
            let a = states[index + 1];
            let b = states[index + 2];
            let c = states[index + 3];
            match op_code {
                99 => break,
                1 => states[c] = states[a] + states[b],
                2 => states[c] = states[a] * states[b],
                _ => {
                    panic!("wrong input")
                }
            }
            index += 4;
        }
        Ok(states[0])
    }
}

impl Solution for Day02 {
    fn test_input() -> String {
        String::from("1,9,10,3,2,3,11,0,99,30,40,50")
    }
    fn solve_part_1(input: String) -> String {
        let mut states: Vec<usize> = Self::parse(input);
        states[1] = 12;
        states[2] = 2;
        Self::simulate(states).unwrap().to_string()
    }
    fn solve_part_2(input: String) -> String {
        let states: Vec<usize> = Self::parse(input);
        for i in 0..99 {
            for j in 0..99 {
                let mut states = states.clone();
                states[1] = i;
                states[2] = j;
                if let Ok(num) = Self::simulate(states) {
                    if num == 19690720 {
                        return (100 * i + j).to_string();
                    }
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
        let states = Day02::parse(input);

        let ans = Day02::simulate(states);
        assert_eq!(ans, Ok(3500))
    }

    #[test]
    fn test_part_2() {
        let input = Day02::test_input();
        let ans = Day02::solve_part_2(input);
        assert_eq!(ans, "0")
    }
}
