use anyhow::Result;
use aoc2021::stream_items_from_file;
use itertools::Itertools;
use std::{
    ops::{Index, IndexMut},
    path::Path,
};

fn parse_lines(input: impl Iterator<Item = String>) -> Vec<usize> {
    input
        .map(|line| {
            line.split(',')
                .filter_map(|crab| Result::ok(crab.parse()))
                .collect()
        })
        .fold1(|mut acc: Vec<_>, mut crab| {
            acc.append(&mut crab);
            acc
        })
        .unwrap_or_default()
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn gauss_fuel_conversion(raw: usize) -> usize {
    raw * (raw + 1) / 2
}

// A simple structure mapping a final alignment position to the total amount of fuel
// It might also be viable to only consider actually existing starting positions for
// better space efficiency, but this was easier to implement.
struct PositionFuelMap(Vec<usize>, usize);

impl Index<usize> for PositionFuelMap {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index - self.1]
    }
}

impl IndexMut<usize> for PositionFuelMap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index - self.1]
    }
}

fn calc_distances<F>(positions: &Vec<usize>, mut fuel_conversion: F) -> PositionFuelMap
where
    F: FnMut(usize) -> usize,
{
    let (min, max) = (
        *positions.iter().min().unwrap(),
        *positions.iter().max().unwrap(),
    );
    let mut output = PositionFuelMap(vec![0; max - min + 1], min);
    for &crab_position in positions {
        for target_position in min..=max {
            output[target_position] += fuel_conversion(abs_diff(crab_position, target_position));
        }
    }

    output
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let crabs = parse_lines(stream_items_from_file(input)?);
    let distances = calc_distances(&crabs, |d| d);
    Ok(*distances.0.iter().min().unwrap())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let crabs = parse_lines(stream_items_from_file(input)?);
    let distances = calc_distances(&crabs, gauss_fuel_conversion);
    Ok(*distances.0.iter().min().unwrap())
}

const INPUT: &str = "input/day07.txt";

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use aoc2021::{stream_items_from_file, test_helpers::create_line_file};
    use tempfile::TempDir;

    use super::*;

    fn example_file() -> (TempDir, impl AsRef<Path>) {
        create_line_file(["16,1,2,0,4,2,7,1,2,14"].iter(), None)
    }

    #[test]
    fn test_parse() {
        let (dir, file) = example_file();
        let crabs = parse_lines(stream_items_from_file::<_, String>(file).unwrap());
        assert_eq!(crabs, vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
        drop(dir);
    }

    #[test]
    fn test_distances_p1() {
        let (dir, file) = example_file();
        let crabs = parse_lines(stream_items_from_file::<_, String>(file).unwrap());
        let distances = calc_distances(&crabs, |d| d);
        assert_eq!(distances[2], 37);
        assert_eq!(distances[1], 41);
        assert_eq!(distances[3], 39);
        assert_eq!(distances[10], 71);
        drop(dir);
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 37);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 168);
        drop(dir);
    }
}
