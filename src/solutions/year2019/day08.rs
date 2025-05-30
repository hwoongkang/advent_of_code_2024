use crate::Solution;

fn parse_with_size(input: String, width: usize, height: usize) -> Vec<String> {
    let len = input.len();
    let size = width * height;
    let mut index = 0;
    let mut ans = vec![];
    while index < len {
        ans.push(input[index..index + size].to_string());
        index += size;
    }
    ans
}

fn solve_with_size(input: String, width: usize, height: usize) -> usize {
    let layers = parse_with_size(input, width, height);
    let count =
        |layer: &str, char: char| -> usize { layer.chars().filter(|&ch| ch == char).count() };
    let layer = layers
        .into_iter()
        .min_by(|a, b| count(a, '0').cmp(&count(b, '0')))
        .unwrap();
    let ones = count(&layer, '1');
    let twos = count(&layer, '2');
    ones * twos
}
fn solve_with_size_2(input: String, width: usize, height: usize) -> String {
    let layers: Vec<Vec<char>> = parse_with_size(input, width, height)
        .into_iter()
        .map(|s| s.chars().collect())
        .collect();
    let mut ans = vec![vec!['2'; width]; height];
    for layer in layers {
        for h in 0..height {
            for w in 0..width {
                if ans[h][w] != '2' {
                    continue;
                } else {
                    ans[h][w] = layer[w + h * width];
                }
            }
        }
    }
    ans.into_iter()
        .map(|v| v.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

pub struct Day08 {}

impl Solution for Day08 {
    fn test_input() -> String {
        String::from("123456789012")
    }
    fn solve_part_1(input: String) -> String {
        solve_with_size(input, 25, 6).to_string()
    }
    fn solve_part_2(input: String) -> String {
        "\n".to_string() + &solve_with_size_2(input, 25, 6)
    }
}

#[cfg(test)]
mod day08_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day08::test_input();
        assert_eq!(solve_with_size(input, 3, 2), 1)
    }

    #[test]
    fn test_part_2() {
        let input = String::from("0222112222120000");
        assert_eq!(solve_with_size_2(input, 2, 2), "01\n10")
    }
}
