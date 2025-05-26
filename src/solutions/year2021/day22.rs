use crate::Solution;
use std::collections::HashSet;

pub struct Day22 {}

impl Day22 {
    fn larger_input() -> String {
        String::from(
            "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682",
        )
    }

    fn part_2_larger() -> String {
        String::from(
            "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507",
        )
    }
}

impl Solution for Day22 {
    fn test_input() -> String {
        String::from(
            "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10",
        )
    }

    fn solve_part_1(input: String) -> String {
        let input: Vec<Cube> = input.lines().filter_map(Cube::from).collect();
        let mut turned_on_cubes: Vec<Cube> = vec![];
        for cube in input {
            let on = cube.on;
            let mut splitted = vec![];
            let mut all_overlaps: Vec<Cube> = vec![];

            for existing in turned_on_cubes {
                let (overlap, mut residue) = existing.split(&cube);
                splitted.append(&mut residue);
                if let Some(overlap) = overlap {
                    all_overlaps.push(overlap);
                }
            }

            if on {
                let mut me = vec![cube];
                for overlap in all_overlaps.iter() {
                    me = me
                        .into_iter()
                        .map(|cube| {
                            let (_, residue) = cube.split(overlap);
                            residue
                        })
                        .flatten()
                        .collect();
                }
                splitted.append(&mut me);
                splitted.append(&mut all_overlaps);
            }

            turned_on_cubes = splitted;
        }
        turned_on_cubes
            .into_iter()
            .map(|cube| cube.size())
            .sum::<i64>()
            .to_string()
    }
    fn solve_part_2(input: String) -> String {
        let input: Vec<Cube> = input.lines().map(Cube::from_raw).collect();
        let mut turned_on_cubes: Vec<Cube> = vec![];
        for cube in input {
            let on = cube.on;
            let mut splitted = vec![];
            let mut all_overlaps: Vec<Cube> = vec![];

            for existing in turned_on_cubes {
                let (overlap, mut residue) = existing.split(&cube);
                splitted.append(&mut residue);
                if let Some(overlap) = overlap {
                    all_overlaps.push(overlap);
                }
            }

            if on {
                let mut me = vec![cube];
                for overlap in all_overlaps.iter() {
                    me = me
                        .into_iter()
                        .map(|cube| {
                            let (_, residue) = cube.split(overlap);
                            residue
                        })
                        .flatten()
                        .collect();
                }
                splitted.append(&mut me);
                splitted.append(&mut all_overlaps);
            }

            turned_on_cubes = splitted;
        }
        turned_on_cubes
            .into_iter()
            .map(|cube| cube.size())
            .sum::<i64>()
            .to_string()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Range {
    min: i64,
    max: i64,
}

impl Range {
    fn size(&self) -> i64 {
        self.max - self.min
    }
    fn new(min: i64, max: i64) -> Self {
        Self { min, max }
    }

    fn from_raw(s: &str) -> Self {
        let range = s.split("=").nth(1).unwrap();
        let mut nums = range.split("..");
        let min = nums.next().unwrap().parse().unwrap();
        let mut max: i64 = nums.next().unwrap().parse().unwrap();
        max += 1;
        Self { min, max }
    }

    fn from(s: &str) -> Option<Self> {
        static CAP: Range = Range { min: -50, max: 51 };
        let range = s.split("=").nth(1).unwrap();
        let mut nums = range.split("..");
        let min = nums.next().unwrap().parse().unwrap();
        let mut max: i64 = nums.next().unwrap().parse().unwrap();
        max += 1;
        let candidate = Self { min, max };
        let overlap = candidate.overlap(&CAP);
        overlap
    }

    fn overlap(&self, other: &Self) -> Option<Self> {
        let min = self.min.max(other.min);
        let max = self.max.min(other.max);
        if min < max {
            Some(Self::new(min, max))
        } else {
            None
        }
    }

    // intersection, difference
    fn split(&self, other: &Range) -> (Option<Range>, Vec<Range>) {
        if let Some(overlap) = self.overlap(other) {
            let mut residue = vec![];
            if self.min < overlap.min {
                residue.push(Range::new(self.min, overlap.min))
            }
            if overlap.max < self.max {
                residue.push(Range::new(overlap.max, self.max))
            }
            (Some(overlap), residue)
        } else {
            (None, vec![self.clone()])
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Cube {
    on: bool,
    x: Range,
    y: Range,
    z: Range,
}

impl Cube {
    fn size(&self) -> i64 {
        self.x.size() * self.y.size() * self.z.size()
    }
    fn from_raw(line: &str) -> Self {
        let mut onoff = line.split_ascii_whitespace();
        let on = onoff.next().unwrap() == "on";
        let mut ranges = onoff.next().unwrap().split(",");
        let x = Range::from_raw(ranges.next().unwrap());
        let y = Range::from_raw(ranges.next().unwrap());
        let z = Range::from_raw(ranges.next().unwrap());
        Self { on, x, y, z }
    }
    fn from(line: &str) -> Option<Self> {
        let mut onoff = line.split_ascii_whitespace();
        let on = onoff.next().unwrap() == "on";
        let mut ranges = onoff.next().unwrap().split(",");
        let x = Range::from(ranges.next().unwrap());
        let y = Range::from(ranges.next().unwrap());
        let z = Range::from(ranges.next().unwrap());
        let Some(x) = x else {
            return None;
        };
        let Some(y) = y else {
            return None;
        };
        let Some(z) = z else {
            return None;
        };
        Some(Self { on, x, y, z })
    }

    fn split(&self, other: &Cube) -> (Option<Cube>, Vec<Cube>) {
        let (x_overlap, mut x_residues) = self.x.split(&other.x);
        let (y_overlap, mut y_residues) = self.y.split(&other.y);
        let (z_overlap, mut z_residues) = self.z.split(&other.z);

        let Some(x_overlap) = x_overlap else {
            return (None, vec![self.clone()]);
        };
        let Some(y_overlap) = y_overlap else {
            return (None, vec![self.clone()]);
        };
        let Some(z_overlap) = z_overlap else {
            return (None, vec![self.clone()]);
        };

        let overlap = Cube {
            on: true,
            x: x_overlap,
            y: y_overlap,
            z: z_overlap,
        };
        let mut residues = vec![];

        x_residues.push(x_overlap);
        y_residues.push(y_overlap);
        z_residues.push(z_overlap);

        for (i, &x) in x_residues.iter().rev().enumerate() {
            for (j, &y) in y_residues.iter().rev().enumerate() {
                for (k, &z) in z_residues.iter().rev().enumerate() {
                    if i + j + k > 0 {
                        residues.push(Cube {
                            on: self.on,
                            x,
                            y,
                            z,
                        })
                    }
                }
            }
        }

        (Some(overlap), residues)
    }
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    #[test]
    fn test_range_split() {
        // No overlap
        let lhs = Range::new(0, 10);
        let rhs = Range::new(10, 20);
        assert_eq!(lhs.split(&rhs), (None, vec![lhs]));

        // complete subset
        let lhs = Range::new(0, 10);
        let rhs = Range::new(-1, 12);
        assert_eq!(lhs.split(&rhs), (Some(lhs.clone()), vec![]));

        // complete subset, vice versa
        assert_eq!(
            rhs.split(&lhs),
            (
                Some(lhs.clone()),
                vec![Range::new(-1, 0), Range::new(10, 12)]
            )
        );

        // partial overlaps
        let lhs = Range::new(0, 10);
        let rhs = Range::new(5, 15);
        let overlap = Range::new(5, 10);
        let lhs_diff = Range::new(0, 5);
        let rhs_diff = Range::new(10, 15);
        assert_eq!(lhs.split(&rhs), (Some(overlap.clone()), vec![lhs_diff]));
        assert_eq!(rhs.split(&lhs), (Some(overlap), vec![rhs_diff]))
    }

    #[test]
    fn test_cube_split() {
        // no overlap
        let lhs = Cube::from("on x=0..1,y=0..1,z=0..1").unwrap();
        let rhs = Cube::from("on x=5..6,y=5..6,z=5..6").unwrap();
        assert_eq!(lhs.split(&rhs), (None, vec![lhs]));

        // complete subset
        let lhs = Cube::from("on x=0..10,y=0..10,z=0..10").unwrap();
        let rhs = Cube::from("on x=5..6,y=5..6,z=5..6").unwrap();

        let (lhs_overlap, residues) = lhs.split(&rhs);
        let lhs_residues: HashSet<Cube> = HashSet::from_iter(residues.into_iter());

        let rhs_overlap = Some(rhs);

        let rhs_residues = HashSet::from([
            Cube::from("on x=0..4,y=0..4,z=0..4").unwrap(),
            Cube::from("on x=0..4,y=0..4,z=5..6").unwrap(),
            Cube::from("on x=0..4,y=0..4,z=7..10").unwrap(),
            Cube::from("on x=0..4,y=5..6,z=0..4").unwrap(),
            Cube::from("on x=0..4,y=5..6,z=5..6").unwrap(),
            Cube::from("on x=0..4,y=5..6,z=7..10").unwrap(),
            Cube::from("on x=0..4,y=7..10,z=0..4").unwrap(),
            Cube::from("on x=0..4,y=7..10,z=5..6").unwrap(),
            Cube::from("on x=0..4,y=7..10,z=7..10").unwrap(),
            Cube::from("on x=5..6,y=0..4,z=0..4").unwrap(),
            Cube::from("on x=5..6,y=0..4,z=5..6").unwrap(),
            Cube::from("on x=5..6,y=0..4,z=7..10").unwrap(),
            Cube::from("on x=5..6,y=5..6,z=0..4").unwrap(),
            Cube::from("on x=5..6,y=5..6,z=7..10").unwrap(),
            Cube::from("on x=5..6,y=7..10,z=0..4").unwrap(),
            Cube::from("on x=5..6,y=7..10,z=5..6").unwrap(),
            Cube::from("on x=5..6,y=7..10,z=7..10").unwrap(),
            Cube::from("on x=7..10,y=0..4,z=0..4").unwrap(),
            Cube::from("on x=7..10,y=0..4,z=5..6").unwrap(),
            Cube::from("on x=7..10,y=0..4,z=7..10").unwrap(),
            Cube::from("on x=7..10,y=5..6,z=0..4").unwrap(),
            Cube::from("on x=7..10,y=5..6,z=5..6").unwrap(),
            Cube::from("on x=7..10,y=5..6,z=7..10").unwrap(),
            Cube::from("on x=7..10,y=7..10,z=0..4").unwrap(),
            Cube::from("on x=7..10,y=7..10,z=5..6").unwrap(),
            Cube::from("on x=7..10,y=7..10,z=7..10").unwrap(),
        ]);

        assert_eq!(lhs_residues, rhs_residues);

        let (lhs_overlap, residues) = rhs.split(&lhs);

        assert_eq!(lhs_overlap, Some(rhs));
        assert_eq!(residues, vec![])
    }

    #[test]
    fn test_part_1() {
        let input = Day22::test_input();
        let ans = Day22::solve_part_1(input);
        assert_eq!(ans, "39")
    }

    #[test]
    fn test_part_1_larger() {
        let input = Day22::larger_input();
        let ans = Day22::solve_part_1(input);
        assert_eq!(ans, "590784")
    }

    #[test]
    fn test_part_2() {
        let input = Day22::part_2_larger();
        let ans = Day22::solve_part_2(input);
        assert_eq!(ans, "2758514936282235")
    }
}
