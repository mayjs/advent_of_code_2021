use anyhow::Result;
use aoc2021::stream_items_from_file;
use itertools::Itertools;
use std::{collections::HashSet, ops::Index, path::Path};

#[derive(Debug)]
struct Heightmap {
    values: Vec<u32>,
    width: usize,
}

impl Heightmap {
    fn parse(lines: impl Iterator<Item = impl AsRef<str>>) -> Self {
        let mut width = 0;
        let values = lines
            .map(|s| {
                s.as_ref()
                    .chars()
                    .map(|vc| vc.to_digit(10).expect("Invalid input char"))
                    .collect()
            })
            .inspect(|parsed_line: &Vec<u32>| width = parsed_line.len())
            .fold1(|mut acc, mut v| {
                acc.append(&mut v);
                acc
            })
            .expect("No lines in input");
        Heightmap { values, width }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.values.len() / self.width
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::with_capacity(4);

        if x != self.width() - 1 {
            res.push((x + 1, y));
        }
        if x != 0 {
            res.push((x - 1, y));
        }

        if y != self.height() - 1 {
            res.push((x, y + 1));
        }
        if y != 0 {
            res.push((x, y - 1));
        }

        res
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let v = self[(x, y)];
        self.neighbors(x, y).into_iter().all(|pos| self[pos] > v)
    }

    fn search_low_points<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        (0..self.width())
            .map(move |x| {
                (0..self.height()).filter_map(move |y| {
                    if self.is_low_point(x, y) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .flatten()
    }

    fn basin_size(&self, x: usize, y: usize) -> usize {
        let mut to_visit = vec![(x, y)];
        let mut visited = HashSet::new();

        let mut counter = 0;

        while let Some((cx, cy)) = to_visit.pop() {
            if visited.contains(&(cx, cy)) {
                continue;
            }
            if self[(cx, cy)] < 9 {
                counter += 1;
                to_visit.extend(self.neighbors(cx, cy));
            }
            visited.insert((cx, cy));
        }

        counter
    }
}

impl Index<(usize, usize)> for Heightmap {
    type Output = u32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        assert!(x < self.width());
        assert!(y < self.height());
        &self.values[x + y * self.width()]
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<u32> {
    let map = Heightmap::parse(stream_items_from_file::<_, String>(input)?);
    Ok(map.search_low_points().map(|(x, y)| map[(x, y)] + 1).sum())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let map = Heightmap::parse(stream_items_from_file::<_, String>(input)?);
    Ok(map
        .search_low_points()
        .map(|(x, y)| map.basin_size(x, y))
        .sorted()
        .rev()
        .take(3)
        .product())
}

const INPUT: &str = "input/day09.txt";

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, path::Path};

    use aoc2021::{stream_items_from_file, test_helpers::create_line_file};
    use indoc::indoc;
    use tempfile::TempDir;

    use super::*;

    fn example_file() -> (TempDir, impl AsRef<Path>) {
        create_line_file(
            [indoc! {"
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678"}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_lowpoints() {
        let (dir, file) = example_file();
        let map = Heightmap::parse(stream_items_from_file::<_, String>(file).unwrap());
        let lowpoints: HashSet<_> = map.search_low_points().collect();
        assert_eq!(
            lowpoints,
            HashSet::from_iter(vec![(1, 0), (9, 0), (2, 2), (6, 4)].into_iter())
        );
        drop(dir);
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 15);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 1134);
        drop(dir);
    }
}
