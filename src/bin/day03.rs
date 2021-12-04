use std::path::Path;

use anyhow::Result;
use aoc2021::stream_items_from_file;
use itertools::iterate;

fn convert_line(line: &str) -> impl Iterator<Item = usize> + '_ {
    line.chars().map(|c| match c {
        '0' => 0,
        '1' => 1,
        _ => panic!("Unexpected value in input"),
    })
}

fn count_digits<I: Iterator<Item = String>>(mut binaries: I) -> (Vec<usize>, usize) {
    let init = convert_line(&binaries.next().expect("Input is empty")).collect();
    binaries.fold((init, 1), |(mut acc, count), next| {
        acc.iter_mut()
            .zip(convert_line(&next))
            .for_each(|(digit_counter, digit)| *digit_counter += digit);
        (acc, count + 1)
    })
}

fn calc_gamma_and_epsilon<I: Iterator<Item = String>>(binaries: I) -> (usize, usize) {
    let (counts, lines) = count_digits(binaries);
    let bitmask = counts.iter().rev().map(|c| match *c > lines / 2 {
        true => 1,
        false => 0,
    });
    iterate(1, |prev| *prev * 2)
        .zip(bitmask)
        .map(|(exp, mask)| (mask * exp, (1 - mask) * exp))
        .fold((0, 0), |(gamma, epsilon), (gn, en)| {
            (gamma + gn, epsilon + en)
        })
}

fn part2_rating(mut binaries: Vec<String>, co2: bool) -> Result<usize> {
    let digits = binaries[0].len();

    for idx in 0..digits {
        // TODO: This `cloned` call should not be necessary, but count_digits expects owned strings...
        let (counts, num) = count_digits(binaries.iter().cloned());
        let count = counts[idx];
        let pat = match (count >= (num + 1) / 2) ^ co2 {
            true => b'1',
            false => b'0',
        };
        binaries.retain(|s| s.as_bytes()[idx] == pat);
        if binaries.len() == 1 {
            return Ok(usize::from_str_radix(&binaries[0], 2)?);
        }
    }
    anyhow::bail!("Invalid search");
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let (gamma, epsilon) = calc_gamma_and_epsilon(stream_items_from_file(input)?);
    Ok(gamma * epsilon)
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let binaries: Vec<String> = stream_items_from_file(input).unwrap().collect();
    let oxygen_rating = part2_rating(binaries.clone(), false)?;
    let co2_rating = part2_rating(binaries.clone(), true)?;
    Ok(oxygen_rating * co2_rating)
}

const INPUT: &str = "input/day03.txt";

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use aoc2021::{stream_items_from_file, test_helpers::create_line_file};
    use tempfile::TempDir;

    fn example_file() -> (TempDir, impl AsRef<Path>) {
        create_line_file(
            [
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_count_digits() {
        let (dir, file) = example_file();
        assert_eq!(
            count_digits(stream_items_from_file(file).unwrap()),
            (vec![7, 5, 8, 7, 5], 12)
        );
        drop(dir);
    }

    #[test]
    fn test_gamma() {
        let (dir, file) = example_file();
        assert_eq!(
            calc_gamma_and_epsilon(stream_items_from_file(file).unwrap()),
            (22, 9)
        );
        drop(dir);
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 198);
        drop(dir);
    }

    #[test]
    fn test_oxygen() {
        let (dir, file) = example_file();
        let input = stream_items_from_file(file).unwrap().collect();
        assert_eq!(part2_rating(input, false).unwrap(), 23);
        drop(dir);
    }

    #[test]
    fn test_co2() {
        let (dir, file) = example_file();
        let input = stream_items_from_file(file).unwrap().collect();
        assert_eq!(part2_rating(input, true).unwrap(), 10);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 230);
        drop(dir);
    }
}
