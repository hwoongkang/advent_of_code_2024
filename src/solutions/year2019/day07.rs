use crate::Solution;

use super::computer::{Computer, Result};

/**
* procedure permutations(k : integer, A : array of any):
   if k = 1 then
       output(A)
   else
       // permutations with last element fixed
       permutations(k - 1, A)
       // permutations with last element swapped out
       for i := 0; i < k-1; i += 1 do
           if k is even then
               swap(A[i], A[k-1])
           else
               swap(A[0], A[k-1])
           end if
           permutations(k - 1, A)
       end for
   end if
*/
fn heaps_algorithm(k: usize, v: &mut Vec<i64>, ans: &mut Vec<Vec<i64>>) {
    if k == 1 {
        ans.push(v.clone());
    } else {
        heaps_algorithm(k - 1, v, ans);
        for i in 0..k - 1 {
            if k % 2 == 0 {
                v.swap(i, k - 1)
            } else {
                v.swap(0, k - 1)
            }
            heaps_algorithm(k - 1, v, ans);
        }
    }
}

fn permutations(n: i64) -> Vec<Vec<i64>> {
    let mut v = (0..n).collect();
    let mut ans = vec![];
    heaps_algorithm(n as usize, &mut v, &mut ans);
    ans
}

fn permutate(mut v: Vec<i64>) -> Vec<Vec<i64>> {
    let mut ans = vec![];
    heaps_algorithm(v.len(), &mut v, &mut ans);
    ans
}

pub struct Day07 {}

impl Solution for Day07 {
    fn test_input() -> String {
        String::new()
    }
    fn solve_part_1(input: String) -> String {
        let simulate = |phase_settings: &[i64]| -> i64 {
            let mut a = Computer::from(&input.clone());
            let mut b = Computer::from(&input.clone());
            let mut c = Computer::from(&input.clone());
            let mut d = Computer::from(&input.clone());
            let mut e = Computer::from(&input.clone());
            a.input_seq = vec![phase_settings[0], 0];
            let output = a.run_until_halt();
            b.input_seq = vec![phase_settings[1], output];
            let output = b.run_until_halt();
            c.input_seq = vec![phase_settings[2], output];
            let output = c.run_until_halt();
            d.input_seq = vec![phase_settings[3], output];
            let output = d.run_until_halt();
            e.input_seq = vec![phase_settings[4], output];
            let output = e.run_until_halt();
            output
        };
        permutations(5)
            .into_iter()
            .map(|p| simulate(&p))
            .max()
            .unwrap()
            .to_string()
    }
    fn solve_part_2(input: String) -> String {
        let simulate = |phase_settings: &[i64]| -> i64 {
            let mut a = Computer::from(&input.clone());
            let mut b = Computer::from(&input.clone());
            let mut c = Computer::from(&input.clone());
            let mut d = Computer::from(&input.clone());
            let mut e = Computer::from(&input.clone());
            a.input_seq = vec![phase_settings[0], 0];
            b.input_seq = vec![phase_settings[1]];
            c.input_seq = vec![phase_settings[2]];
            d.input_seq = vec![phase_settings[3]];
            e.input_seq = vec![phase_settings[4]];
            let mut computers = [a, b, c, d, e];
            let mut computer_index = 0;
            loop {
                let computer = &mut computers[computer_index];

                let result = computer.run();
                match result {
                    Result::Output(output) => {
                        computer_index += 1;
                        computer_index %= 5;
                        computers[computer_index].add_input(output);
                    }
                    Result::Halted(output) => {
                        computer_index += 1;
                        computer_index %= 5;
                        if computer_index == 0 {
                            break output;
                        } else {
                            computers[computer_index].add_input(output);
                        }
                    }
                }
            }
        };
        permutate(vec![5, 6, 7, 8, 9])
            .into_iter()
            .map(|p| simulate(&p))
            .max()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod day07_tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_part_1() {
        let program = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let ans = Day07::solve_part_1(program);
        assert_eq!(ans, "43210");
        let program = String::from(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        let ans = Day07::solve_part_1(program);
        assert_eq!(ans, "54321");
        let program = String::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
        let ans = Day07::solve_part_1(program);
        assert_eq!(ans, "65210")
    }

    #[test]
    fn test_permutations() {
        let ans: HashSet<Vec<i64>> = HashSet::from([
            vec![0, 1, 2],
            vec![0, 2, 1],
            vec![1, 0, 2],
            vec![1, 2, 0],
            vec![2, 0, 1],
            vec![2, 1, 0],
        ]);
        let heap = HashSet::from_iter(permutations(3).into_iter());
        assert_eq!(heap, ans);

        let heap_2 = HashSet::from_iter(permutate(vec![0, 1, 2]).into_iter());
        assert_eq!(heap_2, ans);
    }

    #[test]
    fn test_part_2() {
        let program = String::from(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        let ans = Day07::solve_part_2(program);
        assert_eq!(ans, "139629729")
    }
}
