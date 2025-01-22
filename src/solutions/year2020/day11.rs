use crate::Solution;

pub struct Day11 {}

#[repr(u8)]
#[derive(PartialEq, Eq, Clone)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Seat {
    fn from(char: char) -> Self {
        match char {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => unimplemented!(),
        }
    }
}

#[repr(u8)]
enum Dir {
    N,
    W,
    E,
    S,
    NW,
    NE,
    SW,
    SE,
}

impl Dir {
    fn all() -> [Self; 8] {
        use Dir::*;
        [N, W, E, S, NW, NE, SW, SE]
    }
    fn delta(&self) -> (i32, i32) {
        use Dir::*;
        match self {
            N => (-1, 0),
            S => (1, 0),
            E => (0, 1),
            W => (0, -1),
            NE => (-1, 1),
            NW => (-1, -1),
            SE => (1, 1),
            SW => (1, -1),
        }
    }
}

struct Layout {
    seats: Vec<Vec<Seat>>,
}

impl Layout {
    fn size(&self) -> (usize, usize) {
        (self.seats.len(), self.seats[0].len())
    }
    fn isize(&self) -> (i32, i32) {
        let (r, c) = self.size();
        (r as i32, c as i32)
    }
    fn from(input: String) -> Self {
        Self {
            seats: input
                .lines()
                .map(|line| line.chars().map(Seat::from).collect())
                .collect(),
        }
    }

    fn count_occupied(&self, pos: (usize, usize)) -> u8 {
        let r = pos.0 as i32;
        let c = pos.1 as i32;
        let mut ans = 0;
        let (mr, mc) = self.size();
        let mr = mr as i32;
        let mc = mc as i32;

        for dr in [-1, 0, 1] {
            for dc in [-1, 0, 1] {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let r = r + dr;
                let c = c + dc;
                if r < 0 || c < 0 || r >= mr || c >= mc {
                    continue;
                }
                let r = r as usize;
                let c = c as usize;
                if self.seats[r][c] == Seat::Occupied {
                    ans += 1;
                }
            }
        }
        ans
    }

    fn tick_1(&mut self) -> Option<i32> {
        let mut new_seats = self.seats.clone();
        let (mr, mc) = self.size();
        let mut changed = false;
        for r in 0..mr {
            for c in 0..mc {
                let seat = &self.seats[r][c];
                let count = self.count_occupied((r, c));
                match seat {
                    Seat::Floor => {}
                    Seat::Empty => {
                        if count == 0 {
                            new_seats[r][c] = Seat::Occupied;
                            changed = true;
                        }
                    }
                    Seat::Occupied => {
                        if count >= 4 {
                            new_seats[r][c] = Seat::Empty;
                            changed = true;
                        }
                    }
                }
            }
        }
        if changed {
            self.seats = new_seats;
            None
        } else {
            Some(
                self.seats
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|seat| if seat == &Seat::Occupied { 1 } else { 0 })
                            .sum::<i32>()
                    })
                    .sum(),
            )
        }
    }

    fn scan(&self, pos: (usize, usize), dir: Dir) -> bool {
        let mut r = pos.0 as i32;
        let mut c = pos.1 as i32;
        let (dr, dc) = dir.delta();
        let (mr, mc) = self.isize();
        loop {
            r += dr;
            c += dc;
            if r < 0 || c < 0 || r >= mr || c >= mc {
                break false;
            }
            let r = r as usize;
            let c = c as usize;
            match &self.seats[r][c] {
                Seat::Floor => continue,
                Seat::Occupied => break true,
                Seat::Empty => break false,
            }
        }
    }

    fn scan_occupied(&self, pos: (usize, usize)) -> u8 {
        let mut ans = 0;
        for dir in Dir::all() {
            if self.scan(pos, dir) {
                ans += 1;
            }
        }
        ans
    }

    fn tick_2(&mut self) -> Option<i32> {
        let mut new_seats = self.seats.clone();
        let (mr, mc) = self.size();
        let mut changed = false;
        for r in 0..mr {
            for c in 0..mc {
                let seat = &self.seats[r][c];
                let count = self.scan_occupied((r, c));
                match seat {
                    Seat::Floor => {}
                    Seat::Empty => {
                        if count == 0 {
                            new_seats[r][c] = Seat::Occupied;
                            changed = true;
                        }
                    }
                    Seat::Occupied => {
                        if count >= 5 {
                            new_seats[r][c] = Seat::Empty;
                            changed = true;
                        }
                    }
                }
            }
        }
        if changed {
            self.seats = new_seats;
            None
        } else {
            Some(
                self.seats
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|seat| if seat == &Seat::Occupied { 1 } else { 0 })
                            .sum::<i32>()
                    })
                    .sum(),
            )
        }
    }
}

impl Solution for Day11 {
    fn test_input() -> String {
        String::from(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        )
    }
    fn solve_part_1(input: String) -> String {
        let mut layout = Layout::from(input);
        loop {
            if let Some(output) = layout.tick_1() {
                break output.to_string();
            }
        }
    }
    fn solve_part_2(input: String) -> String {
        let mut layout = Layout::from(input);
        loop {
            if let Some(output) = layout.tick_2() {
                break output.to_string();
            }
        }
    }
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = Day11::test_input();
        let ans = Day11::solve_part_1(input);
        assert_eq!(ans, "37")
    }

    #[test]
    fn test_part_2() {
        let input = Day11::test_input();
        let ans = Day11::solve_part_2(input);
        assert_eq!(ans, "26")
    }

    #[test]
    fn test_scan() {
        let input = ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#....."
            .to_string();

        let layout = Layout::from(input);
        assert_eq!(layout.scan_occupied((4, 3)), 8);
        let input = ".............
.L.L.#.#.#.#.
............."
            .to_string();
        let layout = Layout::from(input);
        assert_eq!(layout.scan_occupied((1, 1)), 0);
        let input = ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##."
            .to_string();
        let layout = Layout::from(input);
        assert_eq!(layout.scan_occupied((3, 3)), 0)
    }
}
