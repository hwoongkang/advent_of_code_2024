use crate::Solution;

const PATTERN: &str = "                  #.
#    ##    ##    ###
 #  #  #  #  #  #  .";

struct NonSquareTile {
    data: Vec<Vec<bool>>,
}

impl NonSquareTile {
    fn _debug(&self) {
        println!("DEBUGGING");
        let (mr, mc) = self.size();
        for r in 0..mr {
            for c in 0..mc {
                print!("{}", if self.data[r][c] { '#' } else { '.' });
            }
            println!()
        }
    }
    fn from(input: &str) -> Self {
        Self {
            data: input
                .lines()
                .map(|line| line.chars().map(|ch| ch == '#').collect())
                .collect(),
        }
    }

    fn rotate(&mut self) {
        let (mr, mc) = self.size();
        let mut new_data = vec![vec![false; mr]; mc];
        for r in 0..mr {
            for c in 0..mc {
                new_data[mc - 1 - c][r] = self.data[r][c];
            }
        }
        self.data = new_data;
    }

    fn flip(&mut self) {
        let (mr, mc) = self.size();
        let hc = mc / 2;
        for r in 0..mr {
            for c in 0..hc {
                self.data[r].swap(c, mc - 1 - c);
            }
        }
    }

    fn size(&self) -> (usize, usize) {
        let mr = self.data.len();
        let mc = self.data[0].len();
        (mr, mc)
    }
}

struct Image {
    data: Vec<Vec<bool>>,
}

impl Image {
    fn size(&self) -> (usize, usize) {
        let mr = self.data.len();
        let mc = self.data[0].len();
        (mr, mc)
    }

    fn try_match(&self, pattern: &NonSquareTile, at: (usize, usize)) -> bool {
        let (mr, mc) = self.size(); // 100
        let (pr, pc) = pattern.size(); // 12
        let (sr, sc) = at;
        if sr + pr > mr || sc + pc > mc {
            return false;
        }
        for x in 0..pr {
            for y in 0..pc {
                let r = sr + x;
                let c = sc + y;
                if pattern.data[x][y] && (!self.data[r][c]) {
                    return false;
                }
            }
        }
        true
    }
    fn find_match(&mut self, pattern: &NonSquareTile) -> usize {
        let (mr, mc) = self.size();
        let (pr, pc) = pattern.size(); // 12

        let mut num_matches = 0;

        for r in 0..mr {
            for c in 0..mc {
                if !self.try_match(pattern, (r, c)) {
                    continue;
                }
                println!("A MATCH {} {}", r, c);
                for x in 0..pr {
                    for y in 0..pc {
                        let r = r + x;
                        let c = c + y;
                        if pattern.data[x][y] {
                            self.data[r][c] = false;
                        }
                    }
                }
                num_matches += 1;
            }
        }
        num_matches
    }

    fn all_matches(&mut self, pattern: &mut NonSquareTile) -> usize {
        let mut num_matches = 0;
        for _ in 0..4 {
            pattern.flip();
            num_matches += self.find_match(pattern);
            pattern.flip();
            num_matches += self.find_match(pattern);
            pattern.rotate();
        }
        num_matches
    }

    fn count(&self) -> usize {
        let (mr, mc) = self.size();
        let mut ans = 0;
        for r in 0..mr {
            for c in 0..mc {
                if self.data[r][c] {
                    ans += 1;
                }
            }
        }
        ans
    }
}

pub struct Day20 {}

impl Solution for Day20 {
    fn test_input() -> String {
        String::from(
            "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...",
        )
    }
    fn solve_part_1(input: String) -> String {
        let tiles = parse_input(input);
        let assembled = assemble_tiles(tiles);
        let size = assembled.len();

        let mut ans = 1;
        ans *= assembled[0][0].id;
        ans *= assembled[0][size - 1].id;
        ans *= assembled[size - 1][size - 1].id;
        ans *= assembled[size - 1][0].id;

        ans.to_string()
    }
    fn solve_part_2(input: String) -> String {
        let tiles = parse_input(input);
        let assembled = assemble_tiles(tiles);
        let size = assembled.len();
        let mut data: Vec<Vec<bool>> = vec![vec![false; 8 * size]; 8 * size];
        println!("SIZE: {}", size);
        for r in 0..size {
            for c in 0..size {
                for x in 0..8 {
                    for y in 0..8 {
                        data[r * 8 + x][c * 8 + y] = assembled[r][c].data[1 + x][1 + y];
                    }
                }
            }
        }
        let mut image = Image { data };
        let mut pattern = NonSquareTile::from(PATTERN);
        let num_matches = image.all_matches(&mut pattern);
        println!("{:?}", num_matches);
        image.count().to_string()
    }
}

#[derive(Copy, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn index(&self) -> usize {
        match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
        }
    }
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Clone)]
struct Tile {
    id: usize,
    data: Vec<Vec<bool>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Edge(u16);

impl Edge {
    fn matches(&self, other: &Self) -> bool {
        self == other
    }
}

impl Tile {
    fn from(lines: &mut std::str::Lines) -> Option<Self> {
        let line = lines.next();
        if line.is_none() {
            return None;
        }
        let line = line.unwrap();
        let l = line.len();
        let id: usize = line[5..l - 1].parse().unwrap();

        let data: Vec<Vec<bool>> = lines
            .take(10)
            .map(|line| line.chars().map(|ch| ch == '#').collect())
            .collect();
        lines.next();

        Some(Self { id, data })
    }

    fn get_edges(&self) -> [Edge; 4] {
        let num = |r: usize, c: usize| -> u16 { self.data[r][c].into() };

        let mut edges = [0; 4];

        for i in 0..10 {
            edges[0] |= num(0, 9 - i) << i;
            edges[1] |= num(9 - i, 9) << i;
            edges[2] |= num(9, 9 - i) << i;
            edges[3] |= num(9 - i, 0) << i;
        }

        [
            Edge(edges[0]),
            Edge(edges[1]),
            Edge(edges[2]),
            Edge(edges[3]),
        ]
    }

    fn rotate(&mut self) {
        for r in 0..5 {
            for c in 0..5 {
                let me = (r, c);
                let first = (c, 9 - r);
                let second = (9 - r, 9 - c);
                let third = (9 - c, r);
                let temp = self.data[me.0][me.1];
                self.data[me.0][me.1] = self.data[first.0][first.1];
                self.data[first.0][first.1] = self.data[second.0][second.1];
                self.data[second.0][second.1] = self.data[third.0][third.1];
                self.data[third.0][third.1] = temp;
            }
        }
    }

    fn flip(&mut self) {
        for r in 0..10 {
            for c in 0..5 {
                self.data[r].swap(c, 9 - c);
            }
        }
    }

    fn try_match(&self, child: &Tile, on: Dir) -> bool {
        let me = self.get_edges();
        let child = child.get_edges();
        me[on.index()].matches(&child[on.opposite().index()])
    }

    fn matches(&self, child: &mut Tile, on: Dir) -> bool {
        for _ in 0..4 {
            child.flip();
            if self.try_match(child, on) {
                return true;
            }

            child.flip();
            if self.try_match(child, on) {
                return true;
            }

            child.rotate();
        }

        false
    }
}

fn parse_input(input: String) -> Vec<Tile> {
    let mut lines = input.lines();
    let mut tiles = vec![];
    while let Some(tile) = Tile::from(&mut lines) {
        tiles.push(tile);
    }
    tiles
}

fn assemble_tiles(mut tiles: Vec<Tile>) -> Vec<Vec<Tile>> {
    let size = if tiles.len() == 9 { 3 } else { 12 };
    let pivot = tiles.remove(3);

    let mut pivot_row = vec![pivot];

    'populate_row: loop {
        let pivot = pivot_row.first().unwrap();
        for (i, tile) in tiles.iter_mut().enumerate() {
            if pivot.matches(tile, Dir::Left) {
                pivot_row.insert(0, tiles.remove(i));
                continue 'populate_row;
            }
        }
        break;
    }

    let mut to_top: Vec<Vec<Tile>> = vec![pivot_row];

    'populate_top: loop {
        let pivot = to_top.first().unwrap().first().unwrap();
        for (i, tile) in tiles.iter_mut().enumerate() {
            if pivot.matches(tile, Dir::Up) {
                let new_row = vec![tiles.remove(i)];
                to_top.insert(0, new_row);
                continue 'populate_top;
            }
        }
        break;
    }

    'populate_bottom: loop {
        let pivot = to_top.last().unwrap().first().unwrap();

        for (i, tile) in tiles.iter_mut().enumerate() {
            if pivot.matches(tile, Dir::Down) {
                let new_row = vec![tiles.remove(i)];
                to_top.push(new_row);
                continue 'populate_bottom;
            }
        }
        break;
    }

    let mut assembled = to_top;

    for r in 0..size {
        let row = &mut assembled[r];
        'populate: while row.len() < size {
            let pivot = row.last().unwrap();
            for (i, tile) in tiles.iter_mut().enumerate() {
                if pivot.matches(tile, Dir::Right) {
                    row.push(tiles.remove(i));
                    continue 'populate;
                }
            }
        }
    }

    println!("ASSEMBLE COMPLETE");
    for r in 0..size {
        for c in 0..size {
            print!("{} ", assembled[r][c].id);
        }
        println!();
    }
    assembled
}
// AB // BA
// DC // CD

// BC // CB
// AD // DA

// CD // DC
// BA // AB

// DA // AD
// CB // BC

#[cfg(test)]
mod day20_tests {
    use super::*;

    #[test]
    fn test_pattern() {
        let mut pattern = NonSquareTile::from(PATTERN);
        pattern._debug();
        pattern.flip();
        pattern._debug();
        pattern.flip();
        pattern.rotate();
        pattern._debug();

        let me = pattern.data.clone();
        let me = NonSquareTile { data: me };
        pattern.flip();
        pattern.flip();
        assert_eq!(me.data, pattern.data);
        for _ in 0..4 {
            pattern.rotate();
        }
        assert_eq!(me.data, pattern.data);
    }

    #[test]
    fn test_tiles() {
        let input = Day20::test_input();
        let mut tiles = parse_input(input);
        assert_eq!(tiles.len(), 9);

        let first_tile = &mut tiles[0];
        let nums = [0b0011010010, 0b0001011001, 0b0011100111, 0b0111110010];

        let edges = [Edge(nums[0]), Edge(nums[1]), Edge(nums[2]), Edge(nums[3])];
        assert_eq!(first_tile.get_edges(), edges);

        let nums = [0b0100101100, 0b0111110010, 0b1110011100, 0b0001011001];

        let edges = [Edge(nums[0]), Edge(nums[1]), Edge(nums[2]), Edge(nums[3])];
        first_tile.flip();
        assert_eq!(first_tile.get_edges(), edges);

        let me = first_tile.clone();
        for _ in 0..4 {
            first_tile.rotate();
        }
        assert_eq!(me.data, first_tile.data);
        first_tile.flip();
        first_tile.flip();
        assert_eq!(me.data, first_tile.data);

        let last_tile = tiles.last().unwrap();
        let nums = [0b1010111110, 0b0100001000, 0b0010111000, 0b1001101000];
        let edges = [Edge(nums[0]), Edge(nums[1]), Edge(nums[2]), Edge(nums[3])];
        assert_eq!(last_tile.get_edges(), edges);
    }

    #[test]
    fn test_part_1() {
        let input = Day20::test_input();
        let ans = Day20::solve_part_1(input);
        assert_eq!(ans, "20899048083289")
    }

    #[test]
    fn test_part_2() {
        let input = Day20::test_input();
        let ans = Day20::solve_part_2(input);
        assert_eq!(ans, "273")
    }
}

// 1951    2311    3079
// 2729    1427    2473
// 2971    1489    1171
