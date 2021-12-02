use anyhow::Result;
use aoc2021::stream_items_from_file;
use std::{num::ParseIntError, ops::Add, path::Path, str::FromStr};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
struct IntVec(isize, isize);

#[derive(Debug, Error)]
enum MovementConversionError {
    #[error("invalid movement")]
    InvalidMovement,
    #[error("invalid syntax")]
    SyntaxError,
    #[error("second part of string is not an int")]
    NoInt(#[from] ParseIntError),
}

impl FromStr for IntVec {
    type Err = MovementConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let dir = parts.next().ok_or(MovementConversionError::SyntaxError)?;
        let amount = isize::from_str(parts.next().ok_or(MovementConversionError::SyntaxError)?)?;
        match dir {
            "forward" => Ok(IntVec(amount, 0)),
            "up" => Ok(IntVec(0, -amount)),
            "down" => Ok(IntVec(0, amount)),
            _ => Err(MovementConversionError::InvalidMovement),
        }
    }
}

impl Add for IntVec {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        IntVec(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl IntVec {
    fn prod(&self) -> isize {
        self.0 * self.1
    }
}

fn apply_movements_part1<I: Iterator<Item = IntVec>>(iter: I) -> IntVec {
    iter.fold(IntVec(0, 0), |acc, x| acc + x)
}

fn apply_movements_part2<I: Iterator<Item = IntVec>>(iter: I) -> IntVec {
    let (x, y, _) = iter.fold((0, 0, 0), |(x, y, aim), command| {
        (x + command.0, y + command.0 * aim, aim + command.1)
    });
    IntVec(x, y)
}

fn part1<P: AsRef<Path>>(input: P) -> Result<isize> {
    let final_pos = apply_movements_part1(stream_items_from_file::<_, IntVec>(input)?);
    Ok(final_pos.prod())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<isize> {
    let final_pos = apply_movements_part2(stream_items_from_file::<_, IntVec>(input)?);
    Ok(final_pos.prod())
}

const INPUT: &str = "input/day02.txt";

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc2021::{stream_items_from_file, test_helpers::create_line_file};
    use tempfile::TempDir;

    use super::*;

    fn example_movements() -> Vec<IntVec> {
        vec![
            IntVec(5, 0),
            IntVec(0, 5),
            IntVec(8, 0),
            IntVec(0, -3),
            IntVec(0, 8),
            IntVec(2, 0),
        ]
    }

    fn example_file() -> (TempDir, impl AsRef<Path>) {
        create_line_file(
            [
                "forward 5",
                "down 5",
                "forward 8",
                "up 3",
                "down 8",
                "forward 2",
            ]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_parse_movements() {
        let (dir, file) = example_file();
        assert_eq!(
            stream_items_from_file::<_, IntVec>(file)
                .unwrap()
                .collect::<Vec<_>>(),
            example_movements()
        );
        drop(dir);
    }

    #[test]
    fn test_apply_movements_part1() {
        let movements = example_movements();
        assert_eq!(apply_movements_part1(movements.into_iter()), IntVec(15, 10));
    }

    #[test]
    fn test_apply_movements_part2() {
        let movements = example_movements();
        assert_eq!(apply_movements_part2(movements.into_iter()), IntVec(15, 60));
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 150);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 900);
        drop(dir);
    }
}
