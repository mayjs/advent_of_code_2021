use std::{path::Path, ops::Add};

use anyhow::Result;
use aoc2021::stream_ints_from_file;
use itertools::Itertools;

const INPUT: &str = "input/day01.txt";

fn number_of_increasing_reads<I: Iterator<Item = usize>>(input: I) -> usize {
    input
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count()
}

fn sum_consecutive_reads<T: Add<Output=T> + Clone>(
    input: impl Iterator<Item = T>,
) -> impl Iterator<Item = T> {
    input
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(number_of_increasing_reads(
        stream_ints_from_file::<_, usize>(input)?,
    ))
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let input_numbers = stream_ints_from_file::<_, usize>(input)?;
    Ok(number_of_increasing_reads(sum_consecutive_reads(
        input_numbers,
    )))
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc2021::test_helpers::create_line_file;

    #[test]
    fn test_d01_examples() {
        let (dir, file) = create_line_file(
            [199, 200, 208, 210, 200, 207, 240, 269, 260, 263].iter(),
            None,
        );
        assert_eq!(part1(&file).unwrap(), 7);
        assert_eq!(part2(&file).unwrap(), 5);
        drop(dir);
    }
}
