use anyhow::Result;
use aoc2021::stream_items_from_file;
use itertools::Itertools;
use std::{
    ops::{BitAnd, Sub},
    path::Path,
    str::FromStr,
};
use thiserror::Error;

#[derive(Debug, Default, Clone)]
struct SignalPattern([bool; 7]);

#[derive(Error, Debug)]
enum SignalPatternStrError {
    #[error("invalid character in signal: {0}")]
    InvalidCharacter(u8),
}

impl FromStr for SignalPattern {
    type Err = SignalPatternStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self::default();
        for signal in s.as_bytes() {
            *result
                .0
                .get_mut((signal - b'a') as usize)
                .ok_or_else(|| SignalPatternStrError::InvalidCharacter(*signal))? = true;
        }
        Ok(result)
    }
}

impl BitAnd for &SignalPattern {
    type Output = SignalPattern;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut output = SignalPattern::default();
        for (i, v) in self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(&l, &r)| l && r)
            .enumerate()
        {
            output.0[i] = v;
        }
        output
    }
}

impl Sub for &SignalPattern {
    type Output = SignalPattern;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut output = SignalPattern::default();
        for (i, v) in self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(&l, &r)| l && !r)
            .enumerate()
        {
            output.0[i] = v;
        }
        output
    }
}

#[derive(Debug, Default)]
struct BaseStore {
    one: SignalPattern,
    four: SignalPattern,
    seven: SignalPattern,
    eight: SignalPattern,
}

impl BaseStore {
    fn from_vec(input: &Vec<SignalPattern>) -> BaseStore {
        let mut output = BaseStore::default();
        for pattern in input {
            match pattern.identify_simple() {
                Some(1) => output.one = pattern.clone(),
                Some(4) => output.four = pattern.clone(),
                Some(7) => output.seven = pattern.clone(),
                Some(8) => output.eight = pattern.clone(),
                _ => (),
            }
        }

        output
    }
}

impl SignalPattern {
    fn count(&self) -> usize {
        self.0.iter().filter(|&&s| s).count()
    }

    fn identify_simple(&self) -> Option<usize> {
        match self.count() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }

    fn identify_deduce(&self, base: &BaseStore) -> usize {
        self.identify_simple()
            .unwrap_or_else(|| match self.count() {
                6 => match (self & &base.one).count() {
                    2 => match (self & &base.four).count() {
                        4 => 9,
                        3 => 0,
                        _ => panic!("Invalid intersection"),
                    },
                    1 => 6,
                    _ => panic!("Invalid intersection"),
                },
                5 => match (self & &(&base.four - &base.one)).count() {
                    2 => 5,
                    1 => match (self & &base.one).count() {
                        1 => 2,
                        2 => 3,
                        _ => panic!("Invalid intersection")
                    },
                    _ => panic!("Invalid intersection"),
                },
                v => panic!("Unexpected count: {}", v)
            })
    }
}

fn parse_line(line: impl AsRef<str>) -> (Vec<SignalPattern>, Vec<SignalPattern>) {
    let mut patterns = line.as_ref().split('|').map(|s| {
        s.split(' ')
            .filter(|s| s.len() > 0)
            .map(|signal| signal.parse::<SignalPattern>())
            .collect::<Result<_, _>>()
            .expect("Error in pattern")
    });
    (
        patterns.next().expect("Missing patterns"),
        patterns.next().expect("Missing examples"),
    )
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(stream_items_from_file(input)?
        .map(|l: String| parse_line(l))
        .map(|(_, example)| example.iter().filter_map(|p| p.identify_simple()).count())
        .sum())
}

fn decode_line(examples: &Vec<SignalPattern>, output: &Vec<SignalPattern>) -> usize {
    let base = BaseStore::from_vec(examples);
    output
        .iter()
        .map(|pattern| pattern.identify_deduce(&base))
        .fold1(|acc, v| (acc * 10) + v)
        .expect("Empty output is not allowed")
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(stream_items_from_file(input)?
        .map(|l: String| parse_line(l))
        .map(|(ex, pat)| decode_line(&ex, &pat))
        .sum())
}

const INPUT: &str = "input/day08.txt";

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
            be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 26);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 61229);
        drop(dir);
    }
}
