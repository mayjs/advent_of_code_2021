use anyhow::Result;
use aoc2021::{
    bidirange::bidi_range,
    stream_items_from_file,
    vec2d::{NumVecParsingError, UVec2D},
};
use itertools::iproduct;
use std::{collections::HashMap, num::ParseIntError, path::Path, str::FromStr};
use thiserror::Error;

#[derive(Debug, PartialEq)]
struct Line {
    start: UVec2D,
    end: UVec2D,
}

impl Line {
    fn is_cardinal(&self) -> bool {
        (self.start.x == self.end.x) ^ (self.start.y == self.end.y)
    }

    fn iter_points(&self) -> Box<dyn Iterator<Item = UVec2D>> {
        if self.is_cardinal() {
            let x = bidi_range(
                self.start.x.try_into().unwrap(),
                self.end.x.try_into().unwrap(),
            );
            let y = bidi_range(
                self.start.y.try_into().unwrap(),
                self.end.y.try_into().unwrap(),
            );
            Box::new(
                iproduct!(x, y)
                    .map(|(x, y)| UVec2D::new(x.try_into().unwrap(), y.try_into().unwrap())),
            )
        } else {
            let x = bidi_range(
                self.start.x.try_into().unwrap(),
                self.end.x.try_into().unwrap(),
            );
            let y = bidi_range(
                self.start.y.try_into().unwrap(),
                self.end.y.try_into().unwrap(),
            );
            Box::new(
                x.zip(y)
                    .map(|(x, y)| UVec2D::new(x.try_into().unwrap(), y.try_into().unwrap())),
            )
        }
    }
}

#[derive(Debug, Error)]
enum LineParsingError {
    #[error("Start or end point is missing")]
    MissingPointError,
    #[error("Could not parse point: {0}")]
    ParseVecError(#[from] NumVecParsingError<ParseIntError>),
}

impl FromStr for Line {
    type Err = LineParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<UVec2D> = s
            .split(" -> ")
            .take(2)
            .map(|s| s.parse::<UVec2D>())
            .collect::<Result<_, _>>()?;
        Ok(Line {
            start: *points.get(0).ok_or(LineParsingError::MissingPointError)?,
            end: *points.get(1).ok_or(LineParsingError::MissingPointError)?,
        })
    }
}

fn mark_overlaps(lines: impl Iterator<Item = Line>) -> impl IntoIterator<Item = (UVec2D, usize)> {
    let mut map = HashMap::<UVec2D, usize>::new();
    lines
        .map(|l| l.iter_points())
        .flatten()
        .for_each(|p| *map.entry(p).or_insert(0) += 1);
    map
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let lines = stream_items_from_file::<_, Line>(input)?.filter(|l| l.is_cardinal());
    let overlaps = mark_overlaps(lines);
    Ok(overlaps.into_iter().map(|t| t.1).filter(|c| *c > 1).count())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let lines = stream_items_from_file::<_, Line>(input)?;
    let overlaps = mark_overlaps(lines);
    Ok(overlaps.into_iter().map(|t| t.1).filter(|c| *c > 1).count())
}

const INPUT: &str = "input/day05.txt";

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use aoc2021::{stream_items_from_file, test_helpers::create_line_file};
    use indoc::indoc;
    use tempfile::TempDir;

    use super::*;

    fn example_file() -> (TempDir, impl AsRef<Path>) {
        create_line_file(
            [indoc! {"
                0,9 -> 5,9
                8,0 -> 0,8
                9,4 -> 3,4
                2,2 -> 2,1
                7,0 -> 7,4
                6,4 -> 2,0
                0,9 -> 2,9
                3,4 -> 1,4
                0,0 -> 8,8
                5,5 -> 8,2
            "}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_line_conversion() {
        let (dir, file) = example_file();
        let first = stream_items_from_file::<_, Line>(file)
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(
            first,
            Line {
                start: UVec2D::new(0, 9),
                end: UVec2D::new(5, 9)
            }
        );
        drop(dir);
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 5);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 12);
        drop(dir);
    }
}
