use anyhow::{anyhow, bail, Result};
use aoc2021::stream_items_from_file;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::fmt::Display;
use std::{ops::Sub, path::Path, str::FromStr};

#[derive(Debug, Clone)]
struct Vertex {
    pos: [i64; 3],
}

#[derive(Debug, Clone)]
struct Cuboid {
    from: Vertex,
    to: Vertex,
}

impl Vertex {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { pos: [x, y, z] }
    }

    fn x(&self) -> i64 {
        self.pos[0]
    }

    fn y(&self) -> i64 {
        self.pos[1]
    }

    fn z(&self) -> i64 {
        self.pos[2]
    }
}

#[derive(Debug, Clone)]
struct Interval(i64, i64);

impl Interval {
    fn contains(&self, value: i64) -> bool {
        value >= self.0 && value <= self.1
    }

    fn intersects(&self, other: &Self) -> bool {
        other.contains(self.0)
            || other.contains(self.1)
            || self.contains(other.0)
            || self.contains(other.1)
    }

    fn is_valid(&self) -> bool {
        self.0 <= self.1
    }

    fn clamp(&self, other: &Interval) -> Interval {
        Interval(cmp::max(self.0, other.0), cmp::min(self.1, other.1))
    }

    fn len(&self) -> usize {
        (self.1 - self.0 + 1) as usize
    }
}

impl Sub for &Interval {
    type Output = Vec<Interval>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Vec::new();
        if self.0 < rhs.0 {
            result.push(Interval(self.0, rhs.0 - 1));
        }
        if self.1 > rhs.1 {
            result.push(Interval(rhs.1 + 1, self.1));
        }
        result
    }
}

impl Sub for Interval {
    type Output = Vec<Interval>;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl FromStr for Interval {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[\-\d]+").unwrap();
        }
        let values = RE
            .find_iter(s)
            .take(2)
            .map(|s| s.as_str().parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(
            *values
                .get(0)
                .ok_or(anyhow!("Missing value in interval descriptor {}", s))?,
            *values
                .get(1)
                .ok_or(anyhow!("Missing value in interval descriptor {}", s))?,
        ))
    }
}

impl Cuboid {
    fn from_intervals(x_interval: &Interval, y_interval: &Interval, z_interval: &Interval) -> Self {
        Cuboid {
            from: Vertex::new(x_interval.0, y_interval.0, z_interval.0),
            to: Vertex::new(x_interval.1, y_interval.1, z_interval.1),
        }
    }

    fn x_interval(&self) -> Interval {
        Interval(self.from.x(), self.to.x())
    }

    fn y_interval(&self) -> Interval {
        Interval(self.from.y(), self.to.y())
    }

    fn z_interval(&self) -> Interval {
        Interval(self.from.z(), self.to.z())
    }

    fn intersects(&self, other: &Self) -> bool {
        self.x_interval().intersects(&other.x_interval())
            && self.y_interval().intersects(&other.y_interval())
            && self.z_interval().intersects(&other.z_interval())
    }

    fn volume(&self) -> i64 {
        (self.to.x() - self.from.x() + 1)
            * (self.to.y() - self.from.y() + 1)
            * (self.to.z() - self.from.z() + 1)
    }
}

impl Sub for &Cuboid {
    type Output = Vec<Cuboid>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Vec::new();

        if self.x_interval().contains(rhs.x_interval().0) {
            let xi = Interval(self.from.x(), rhs.from.x() - 1);
            if xi.is_valid() {
                res.push(Cuboid::from_intervals(
                    &xi,
                    &self.y_interval(),
                    &self.z_interval(),
                ));
            }
        }

        if self.y_interval().contains(rhs.y_interval().0) {
            let yi = Interval(self.from.y(), rhs.from.y() - 1);
            if yi.is_valid() {
                res.push(Cuboid::from_intervals(
                    &rhs.x_interval().clamp(&self.x_interval()),
                    &yi,
                    &rhs.z_interval().clamp(&self.z_interval()),
                ));
            }
        }

        if self.z_interval().contains(rhs.z_interval().0) {
            let zi = Interval(self.from.z(), rhs.from.z() - 1);
            if zi.is_valid() {
                res.push(Cuboid::from_intervals(
                    &rhs.x_interval().clamp(&self.x_interval()),
                    &self.y_interval(),
                    &zi,
                ));
            }
        }

        if self.x_interval().contains(rhs.x_interval().1) {
            let xi = Interval(rhs.to.x() + 1, self.to.x());
            if xi.is_valid() {
                res.push(Cuboid::from_intervals(
                    &xi,
                    &self.y_interval(),
                    &self.z_interval(),
                ));
            }
        }

        if self.y_interval().contains(rhs.y_interval().1) {
            let yi = Interval(rhs.to.y() + 1, self.to.y());
            if yi.is_valid() {
                res.push(Cuboid::from_intervals(
                    &rhs.x_interval().clamp(&self.x_interval()),
                    &yi,
                    &rhs.z_interval().clamp(&self.z_interval()),
                ));
            }
        }

        if self.z_interval().contains(rhs.z_interval().1) {
            let zi = Interval(rhs.to.z() + 1, self.to.z());
            if zi.is_valid() {
                res.push(Cuboid::from_intervals(
                    &rhs.x_interval().clamp(&self.x_interval()),
                    &self.y_interval(),
                    &zi,
                ));
            }
        }

        // println!("{} - {} -> {} results", self, rhs, res.len());

        res
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Action {
    On,
    Off,
}

fn parse_action(descriptor: String) -> Result<(Action, Cuboid)> {
    lazy_static! {
        static ref INTERVAL_RE: Regex = Regex::new(r"[\-\d]+..[\-\d]+").unwrap();
    }
    let action = if descriptor.starts_with("on") {
        Action::On
    } else {
        Action::Off
    };

    let intervals = INTERVAL_RE.find_iter(&descriptor).take(3).collect_vec();
    if intervals.len() != 3 {
        bail!(
            "Wrong number of intervals (Wanted 3, got {} in input {})",
            intervals.len(),
            descriptor
        );
    }
    let xi = Interval::from_str(intervals[0].as_str())?;
    let yi = Interval::from_str(intervals[1].as_str())?;
    let zi = Interval::from_str(intervals[2].as_str())?;

    Ok((action, Cuboid::from_intervals(&xi, &yi, &zi)))
}

fn execute_action(mut cuboids: Vec<Cuboid>, action: Action, new_cuboid: &Cuboid) -> Vec<Cuboid> {
    match action {
        Action::On => {
            let mut resulting_cuboids = vec![new_cuboid.clone()];
            for old_cuboid in cuboids.iter() {
                resulting_cuboids = execute_action(resulting_cuboids, Action::Off, old_cuboid)
            }
            cuboids.append(&mut resulting_cuboids);
            cuboids
        }
        Action::Off => {
            let mut resulting_cuboids = Vec::with_capacity(cuboids.len());
            for cuboid in cuboids {
                if !cuboid.intersects(new_cuboid) {
                    resulting_cuboids.push(cuboid);
                } else {
                    resulting_cuboids.append(&mut (&cuboid - new_cuboid));
                }
            }
            resulting_cuboids
        }
    }
}

#[allow(dead_code)]
fn scadviz(input: &Vec<Cuboid>) {
    for cuboid in input {
        println!(
            "translate([{},{},{}])",
            cuboid.from.x() * 10,
            cuboid.from.y() * 10,
            cuboid.from.z() * 10
        );
        println!(
            "cube([{},{},{}]);",
            cuboid.x_interval().len() * 10,
            cuboid.y_interval().len() * 10,
            cuboid.z_interval().len() * 10
        );
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.0, self.1)
    }
}

impl Display for Cuboid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x={},y={},z={}",
            self.x_interval(),
            self.y_interval(),
            self.z_interval()
        )
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<i64> {
    let init_interval = Interval(-50, 50);
    let cuboids = stream_items_from_file(input)?
        .map(parse_action)
        .map(|maybe_action| maybe_action.expect("Parsing failed"))
        .filter(|(_, cuboid)| {
            [
                cuboid.from.x(),
                cuboid.from.y(),
                cuboid.from.z(),
                cuboid.to.x(),
                cuboid.to.y(),
                cuboid.to.z(),
            ]
            .iter()
            .all(|p| init_interval.contains(*p))
        })
        .fold(Vec::new(), |acc, (action, new_cuboid)| {
            execute_action(acc, action, &new_cuboid)
        });

    // scadviz(&cuboids);

    Ok(cuboids.iter().map(Cuboid::volume).sum())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<i64> {
    let cuboids = stream_items_from_file(input)?
        .map(parse_action)
        .map(|maybe_action| maybe_action.expect("Parsing failed"))
        .fold(Vec::new(), |acc, (action, new_cuboid)| {
            execute_action(acc, action, &new_cuboid)
        });

    // scadviz(&cuboids);

    Ok(cuboids.iter().map(Cuboid::volume).sum())
}

const INPUT: &str = "input/day22.txt";

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use aoc2021::test_helpers::create_line_file;
    use indoc::indoc;
    use tempfile::TempDir;

    use super::*;

    fn example_file() -> (TempDir, impl AsRef<Path>) {
        create_line_file(
            [indoc! {"
                on x=-20..26,y=-36..17,z=-47..7
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
                on x=967..23432,y=45373..81175,z=27513..53682"}]
            .iter(),
            None,
        )
    }

    fn example_file_small() -> (TempDir, impl AsRef<Path>) {
        create_line_file(
            [indoc! {"
                on x=10..12,y=10..12,z=10..12
                on x=11..13,y=11..13,z=11..13
                off x=9..11,y=9..11,z=9..11
                on x=10..10,y=10..10,z=10..10"}]
            .iter(),
            None,
        )
    }

    fn example_file_very_small() -> (TempDir, impl AsRef<Path>) {
        create_line_file(
            [indoc! {"
                on x=0..3,y=0..2,z=0..2
                off x=0..3,y=0..0,z=0..0
                off x=0..3,y=2..2,z=2..2
                on x=5..9,y=0..0,z=0..0
                off x=6..8,y=0..0,z=0..0
                off x=1..2,y=0..2,z=0..2"}]
            .iter(),
            None,
        )
    }

    fn example_file_xlarge() -> (TempDir, impl AsRef<Path>) {
        create_line_file(
            [indoc! {"
            on x=-5..47,y=-31..22,z=-19..33
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
            off x=-93533..-4276,y=-16170..68771,z=-104985..-24507"}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file_very_small();
        assert_eq!(part1(file).unwrap(), 16);
        drop(dir);
        let (dir, file) = example_file_small();
        assert_eq!(part1(file).unwrap(), 39);
        drop(dir);
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 590784);
        drop(dir);
        let (dir, file) = example_file_xlarge();
        assert_eq!(part1(file).unwrap(), 474140);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file_xlarge();
        assert_eq!(part2(file).unwrap(), 2758514936282235);
        drop(dir);
    }
}
