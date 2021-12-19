use anyhow::anyhow;
use anyhow::Result;
use aoc2021::stream_file_blocks;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Mul, Sub},
    path::Path,
    str::FromStr,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Transform {
    indices: [usize; 3],
    factors: [i32; 3],
}

lazy_static! {
    static ref CARDINAL_TRANSFORMS: Vec<Transform> = {

        let factors = &[-1,1];
        let mut res = Vec::new();
        for i1 in 0..=2 {
            for i2 in 0..=2 {
                if i2 == i1 {
                    continue;
                }
                for i3 in 0..=2 {
                    if i3 == i2 || i3 == i1 {
                        continue;
                    }
                    res.extend(factors.iter().cartesian_product(factors).cartesian_product(factors).map(|((&f1,&f2),&f3)| {
                        Transform { indices: [i1,i2,i3], factors: [f1,f2,f3]}
                    }));
                }
            }
        }

        res
    };
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
struct Vec3D {
    coords: [i32; 3],
}

impl Vec3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { coords: [x, y, z] }
    }
}

impl Mul<&Vec3D> for &Transform {
    type Output = Vec3D;

    fn mul(self, rhs: &Vec3D) -> Self::Output {
        Vec3D::new(self.factors[0]*rhs.coords[self.indices[0]], self.factors[1]*rhs.coords[self.indices[1]], self.factors[2]*rhs.coords[self.indices[2]])
    }
}

impl Sub for &Vec3D {
    type Output = Vec3D;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut coords_iter = self.coords.iter().zip(rhs.coords).map(|(l, r)| l - r);
        let coords = [
            coords_iter.next().unwrap(),
            coords_iter.next().unwrap(),
            coords_iter.next().unwrap(),
        ];
        Vec3D { coords }
    }
}

impl Add for &Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: Self) -> Self::Output {
        let mut coords_iter = self.coords.iter().zip(rhs.coords).map(|(l, r)| l + r);
        let coords = [
            coords_iter.next().unwrap(),
            coords_iter.next().unwrap(),
            coords_iter.next().unwrap(),
        ];
        Vec3D { coords }
    }
}

impl Vec3D {
    fn manhatten_value(&self) -> i32 {
        self.coords.iter().map(|v| v.abs()).sum()
    }
}

impl FromStr for Vec3D {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[\-\d]+").unwrap();
        }
        let values = RE
            .find_iter(s)
            .take(3)
            .map(|s| s.as_str().parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Vec3D::new(
            *values.get(0).ok_or(anyhow!("Missing value"))?,
            *values.get(1).ok_or(anyhow!("Missing value"))?,
            *values.get(2).ok_or(anyhow!("Missing value"))?,
        ))
    }
}

fn find_transformation(
    baseline: &HashSet<Vec3D>,
    to_match: &HashSet<Vec3D>,
) -> Option<(Transform, Vec3D)> {
    for transform in CARDINAL_TRANSFORMS.iter() {
        let mut distance_counts: HashMap<Vec3D, usize> = HashMap::new();
        to_match
            .iter()
            .map(|relative_beacon| transform * relative_beacon)
            .cartesian_product(baseline.iter())
            .map(|(candidate, baseline)| baseline - &candidate)
            .for_each(|dist| *distance_counts.entry(dist).or_insert(0) += 1);

        for (offset, count) in distance_counts {
            if count >= 12 {
                return Some((transform.clone(), offset));
            }
        }
    }
    None
}

fn assemble_map(mut relative_positions: Vec<HashSet<Vec3D>>) -> (HashSet<Vec3D>, HashSet<Vec3D>) {
    // Initial Baseline is what the first scanner sees
    let mut map = relative_positions.remove(0);
    let mut scanner_map = HashSet::new();
    scanner_map.insert(Vec3D::new(0,0,0));
    let mut to_remove: Vec<usize> = Vec::new();
    while relative_positions.len() > 0 {
        for i in 0..relative_positions.len() {
            let scanner_result = &relative_positions[i];
            if let Some((transform, offset)) = find_transformation(&map, scanner_result) {
                map.extend(
                    scanner_result
                        .iter()
                        .map(|rel_beacon| &(&transform * rel_beacon) + &offset)
                );
                to_remove.push(i);

                scanner_map.insert(offset);
            }
        }
        if to_remove.len() == 0 {
            panic!(
                "No progress possible, number of scanners left: {}",
                relative_positions.len()
            );
        }
        while let Some(i) = to_remove.pop() {
            relative_positions.remove(i);
        }
    }
    (map, scanner_map)
}

fn parse_beacon_positions<P: AsRef<Path>>(input: P) -> Result<Vec<HashSet<Vec3D>>> {
    Ok(stream_file_blocks(input)?
        .map(|scanner_data| {
            scanner_data[1..]
                .iter()
                .map(|line| line.parse::<Vec3D>().unwrap())
                .collect()
        })
        .collect())
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let scanner_results = parse_beacon_positions(input)?;
    let (map, _) = assemble_map(scanner_results);
    Ok(map.len())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<i32> {
    let scanner_results = parse_beacon_positions(input)?;
    let (_,map) = assemble_map(scanner_results);

    let max_dist = map.iter().cartesian_product(map.iter()).map(|(v1, v2)| (v2 - v1).manhatten_value()).max().unwrap();
    Ok(max_dist)
}

const INPUT: &str = "input/day19.txt";

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
                --- scanner 0 ---
                404,-588,-901
                528,-643,409
                -838,591,734
                390,-675,-793
                -537,-823,-458
                -485,-357,347
                -345,-311,381
                -661,-816,-575
                -876,649,763
                -618,-824,-621
                553,345,-567
                474,580,667
                -447,-329,318
                -584,868,-557
                544,-627,-890
                564,392,-477
                455,729,728
                -892,524,684
                -689,845,-530
                423,-701,434
                7,-33,-71
                630,319,-379
                443,580,662
                -789,900,-551
                459,-707,401
                
                --- scanner 1 ---
                686,422,578
                605,423,415
                515,917,-361
                -336,658,858
                95,138,22
                -476,619,847
                -340,-569,-846
                567,-361,727
                -460,603,-452
                669,-402,600
                729,430,532
                -500,-761,534
                -322,571,750
                -466,-666,-811
                -429,-592,574
                -355,545,-477
                703,-491,-529
                -328,-685,520
                413,935,-424
                -391,539,-444
                586,-435,557
                -364,-763,-893
                807,-499,-711
                755,-354,-619
                553,889,-390
                
                --- scanner 2 ---
                649,640,665
                682,-795,504
                -784,533,-524
                -644,584,-595
                -588,-843,648
                -30,6,44
                -674,560,763
                500,723,-460
                609,671,-379
                -555,-800,653
                -675,-892,-343
                697,-426,-610
                578,704,681
                493,664,-388
                -671,-858,530
                -667,343,800
                571,-461,-707
                -138,-166,112
                -889,563,-600
                646,-828,498
                640,759,510
                -630,509,768
                -681,-892,-333
                673,-379,-804
                -742,-814,-386
                577,-820,562
                
                --- scanner 3 ---
                -589,542,597
                605,-692,669
                -500,565,-823
                -660,373,557
                -458,-679,-417
                -488,449,543
                -626,468,-788
                338,-750,-386
                528,-832,-391
                562,-778,733
                -938,-730,414
                543,643,-506
                -524,371,-870
                407,773,750
                -104,29,83
                378,-903,-323
                -778,-728,485
                426,699,580
                -438,-605,-362
                -469,-447,-387
                509,732,623
                647,635,-688
                -868,-804,481
                614,-800,639
                595,780,-596
                
                --- scanner 4 ---
                727,592,562
                -293,-554,779
                441,611,-461
                -714,465,-776
                -743,427,-804
                -660,-479,-426
                832,-632,460
                927,-485,-438
                408,393,-506
                466,436,-512
                110,16,151
                -258,-428,682
                -393,719,612
                -211,-452,876
                808,-476,-593
                -575,615,604
                -485,667,467
                -680,325,-822
                -627,-443,-432
                872,-547,-609
                833,512,582
                807,604,487
                839,-516,451
                891,-625,532
                -652,-548,-490
                30,-46,-14
                "}]
            .iter(),
            None,
        )
    }

    fn example_beacons() -> HashSet<Vec3D> {
        let input: &str = indoc! {"
            -892,524,684
            -876,649,763
            -838,591,734
            -789,900,-551
            -739,-1745,668
            -706,-3180,-659
            -697,-3072,-689
            -689,845,-530
            -687,-1600,576
            -661,-816,-575
            -654,-3158,-753
            -635,-1737,486
            -631,-672,1502
            -624,-1620,1868
            -620,-3212,371
            -618,-824,-621
            -612,-1695,1788
            -601,-1648,-643
            -584,868,-557
            -537,-823,-458
            -532,-1715,1894
            -518,-1681,-600
            -499,-1607,-770
            -485,-357,347
            -470,-3283,303
            -456,-621,1527
            -447,-329,318
            -430,-3130,366
            -413,-627,1469
            -345,-311,381
            -36,-1284,1171
            -27,-1108,-65
            7,-33,-71
            12,-2351,-103
            26,-1119,1091
            346,-2985,342
            366,-3059,397
            377,-2827,367
            390,-675,-793
            396,-1931,-563
            404,-588,-901
            408,-1815,803
            423,-701,434
            432,-2009,850
            443,580,662
            455,729,728
            456,-540,1869
            459,-707,401
            465,-695,1988
            474,580,667
            496,-1584,1900
            497,-1838,-617
            527,-524,1933
            528,-643,409
            534,-1912,768
            544,-627,-890
            553,345,-567
            564,392,-477
            568,-2007,-577
            605,-1665,1952
            612,-1593,1893
            630,319,-379
            686,-3108,-505
            776,-3184,-501
            846,-3110,-434
            1135,-1161,1235
            1243,-1093,1063
            1660,-552,429
            1693,-557,386
            1735,-437,1738
            1749,-1800,1813
            1772,-405,1572
            1776,-675,371
            1779,-442,1789
            1780,-1548,337
            1786,-1538,337
            1847,-1591,415
            1889,-1729,1762
            1994,-1805,1792"
        };
        input.lines().map(Vec3D::from_str).collect::<Result<_,_>>().unwrap()
    } 

    #[test]
    fn test_card_transforms() {
        // This fails, how do you get to 24 transformations?
        assert_eq!(
            CARDINAL_TRANSFORMS
                .iter()
                .cloned()
                .collect::<HashSet<_>>()
                .len(),
            24
        );
    }

    #[test]
    fn test_correlation_checks() {
        let (dir, file) = example_file();
        let scanner_results = parse_beacon_positions(file).unwrap();
        let (map,_) = assemble_map(scanner_results);

        let superset = example_beacons();
        assert!(map == superset);

        drop(dir);
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 79);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 3621);
        drop(dir);
    }
}
