use crate::Solution;

type ConwaySpace = Vec<Vec<Vec<bool>>>; // x y z

type C4Space = Vec<Vec<Vec<Vec<bool>>>>;

struct C4 {
    cells: C4Space,
}

impl C4 {
    fn from(input: String) -> Self {
        let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mx = chars.len();
        let my = chars[0].len();
        let mz = 1;
        let mw = 1;
        let mut cells = vec![vec![vec![vec![false; mw]; mz]; my]; mx];
        for x in 0..mx {
            for y in 0..my {
                cells[x][y][0][0] = chars[x][y] == '#';
            }
        }
        Self { cells }
    }

    fn size(&self) -> (usize, usize, usize, usize) {
        let mx = self.cells.len();
        let my = self.cells[0].len();
        let mz = self.cells[0][0].len();
        let mw = self.cells[0][0][0].len();
        (mx, my, mz, mw)
    }

    fn isize(&self) -> (i32, i32, i32, i32) {
        let (mx, my, mz, mw) = self.size();
        (mx as i32, my as i32, mz as i32, mw as i32)
    }

    fn increase_size(&mut self) {
        let (mx, my, mz, mw) = self.size();

        let mut new_space: C4Space = vec![vec![vec![vec![false; mw + 2]; mz + 2]; my + 2]; mx + 2];

        for x in 0..mx {
            for y in 0..my {
                for z in 0..mz {
                    for w in 0..mw {
                        new_space[x + 1][y + 1][z + 1][w + 1] = self.cells[x][y][z][w];
                    }
                }
            }
        }

        self.cells = new_space;
    }

    fn num_active(&self, pos: (usize, usize, usize, usize)) -> usize {
        let (x, y, z, w) = pos;
        let me = if self.cells[x][y][z][w] { 1 } else { 0 };

        let x = x as i32;
        let y = y as i32;
        let z = z as i32;
        let w = w as i32;

        let mut ans = 0;

        let (mx, my, mz, mw) = self.isize();
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                for dz in [-1, 0, 1] {
                    for dw in [-1, 0, 1] {
                        let x = x + dx;
                        let y = y + dy;
                        let z = z + dz;
                        let w = w + dw;

                        if (x < 0 || x >= mx)
                            || (y < 0 || y >= my)
                            || (z < 0 || z >= mz)
                            || (w < 0 || w >= mw)
                        {
                            continue;
                        }
                        let x = x as usize;
                        let y = y as usize;
                        let z = z as usize;
                        let w = w as usize;
                        if self.cells[x][y][z][w] {
                            ans += 1;
                        }
                    }
                }
            }
        }
        ans - me
    }
    fn tick(&mut self) -> usize {
        self.increase_size();
        let mut new_cells = self.cells.clone();
        let (mx, my, mz, mw) = self.size();
        for x in 0..mx {
            for y in 0..my {
                for z in 0..mz {
                    for w in 0..mw {
                        let count = self.num_active((x, y, z, w));
                        if self.cells[x][y][z][w] && (count < 2 || count > 3) {
                            new_cells[x][y][z][w] = false;
                        }
                        if !self.cells[x][y][z][w] && count == 3 {
                            new_cells[x][y][z][w] = true;
                        }
                    }
                }
            }
        }
        self.cells = new_cells;
        self.cells
            .iter()
            .map(|yzw| {
                yzw.iter()
                    .map(|zw| {
                        zw.iter()
                            .map(|w| w.iter().map(|b| if *b { 1 } else { 0 }).sum::<usize>())
                            .sum::<usize>()
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

struct Conway {
    cells: ConwaySpace,
}

impl Conway {
    fn from(input: String) -> Self {
        let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mx = chars.len();
        let my = chars[0].len();
        let mz = 1;
        let mut cells = vec![vec![vec![false; mz]; my]; mx];
        for x in 0..mx {
            for y in 0..my {
                cells[x][y][0] = chars[x][y] == '#';
            }
        }
        Self { cells }
    }

    fn increase_size(&mut self) {
        let (mx, my, mz) = self.size();

        let mut new_space: ConwaySpace = vec![vec![vec![false; mz + 2]; my + 2]; mx + 2];

        for x in 0..mx {
            for y in 0..my {
                for z in 0..mz {
                    new_space[x + 1][y + 1][z + 1] = self.cells[x][y][z];
                }
            }
        }

        self.cells = new_space;
    }

    fn size(&self) -> (usize, usize, usize) {
        let mx = self.cells.len();
        let my = self.cells[0].len();
        let mz = self.cells[0][0].len();
        (mx, my, mz)
    }

    fn isize(&self) -> (i32, i32, i32) {
        let (mx, my, mz) = self.size();
        (mx as i32, my as i32, mz as i32)
    }

    fn num_active(&self, pos: (usize, usize, usize)) -> usize {
        let x = pos.0 as i32;
        let y = pos.1 as i32;
        let z = pos.2 as i32;
        let me = if self.cells[pos.0][pos.1][pos.2] {
            1
        } else {
            0
        };
        let mut ans = 0;

        let (mx, my, mz) = self.isize();
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                for dz in [-1, 0, 1] {
                    let x = x + dx;
                    let y = y + dy;
                    let z = z + dz;

                    if (x < 0 || x >= mx) || (y < 0 || y >= my) || (z < 0 || z >= mz) {
                        continue;
                    }
                    let x = x as usize;
                    let y = y as usize;
                    let z = z as usize;
                    if self.cells[x][y][z] {
                        ans += 1;
                    }
                }
            }
        }
        ans - me
    }

    fn tick(&mut self) -> usize {
        self.increase_size();
        let mut new_cells = self.cells.clone();
        let (mx, my, mz) = self.size();
        for x in 0..mx {
            for y in 0..my {
                for z in 0..mz {
                    let count = self.num_active((x, y, z));
                    if self.cells[x][y][z] && (count < 2 || count > 3) {
                        new_cells[x][y][z] = false;
                    }
                    if !self.cells[x][y][z] && count == 3 {
                        new_cells[x][y][z] = true;
                    }
                }
            }
        }
        self.cells = new_cells;
        self.cells
            .iter()
            .map(|yz| {
                yz.iter()
                    .map(|z| z.iter().map(|b| if *b { 1 } else { 0 }).sum::<usize>())
                    .sum::<usize>()
            })
            .sum()
    }
}

pub struct Day17 {}

impl Solution for Day17 {
    fn test_input() -> String {
        String::from(
            ".#.
..#
###",
        )
    }

    fn solve_part_1(input: String) -> String {
        let mut space = Conway::from(input);
        let mut output = 0;
        for _ in 0..6 {
            output = space.tick();
        }
        output.to_string()
    }

    fn solve_part_2(input: String) -> String {
        let mut space = C4::from(input);
        let mut output = 0;
        for i in 0..6 {
            println!("tick #{}", i + 1);
            output = space.tick();
        }
        output.to_string()
    }
}
#[cfg(test)]
mod day17_tests {
    use super::*;

    #[test]
    fn test_tick() {
        let input = String::from(
            ".#.
..#
###",
        );
        let mut space = Conway::from(input);
        let ans = space.tick();
        assert_eq!(ans, 11);
    }

    #[test]
    fn test_part_1() {
        let input = Day17::test_input();
        let ans = Day17::solve_part_1(input);
        assert_eq!(ans, "112");
    }

    #[test]
    fn test_part_2() {
        let input = Day17::test_input();
        let ans = Day17::solve_part_2(input);
        assert_eq!(ans, "848");
    }
}
