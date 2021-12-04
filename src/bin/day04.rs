use std::{
    cmp::Ordering,
    num::ParseIntError,
    ops::{Index, IndexMut},
    path::Path,
};

use anyhow::Result;
use aoc2021::stream_file_blocks;
use regex::Regex;

fn get_draws(line: &str) -> Vec<usize> {
    line.split(',')
        .map(|s| s.parse::<usize>())
        .collect::<Result<_, _>>()
        .unwrap()
}

struct BingoField {
    content: Vec<(usize, bool)>,
    width: usize,
}

impl TryFrom<Vec<String>> for BingoField {
    type Error = ParseIntError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let delim_regex = Regex::new(r"\s+").unwrap();

        let width = delim_regex.split(&value[0]).count();
        let content = value
            .iter()
            .map(|line| {
                delim_regex
                    .split(line)
                    .filter(|p| p.len() > 0)
                    .map(|s| s.parse::<usize>())
            })
            .flatten()
            .collect::<Result<Vec<usize>, _>>()
            .unwrap()
            .into_iter()
            .map(|v| (v, false))
            .collect();

        Ok(BingoField { content, width })
    }
}

impl Index<(usize, usize)> for BingoField {
    type Output = (usize, bool);

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.content[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for BingoField {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.content[y * self.width + x]
    }
}

impl BingoField {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.content.len() / self.width
    }

    fn is_won(&self) -> bool {
        (0..self.width())
            .map(|x| (0..self.height()).map(|y| self[(x, y)].1).all(|b| b))
            .any(|b| b)
            || (0..self.height())
                .map(|y| (0..self.width()).map(|x| self[(x, y)].1).all(|b| b))
                .any(|b| b)
    }

    fn base_score(&self) -> usize {
        self.content.iter().filter(|(_, m)| !m).map(|t| t.0).sum()
    }

    fn mark(&mut self, num: usize) {
        self.content
            .iter_mut()
            .filter(|(n, _)| *n == num)
            .next()
            .map(|t| t.1 = true);
    }

    fn score_with_draws(&mut self, draws: impl Iterator<Item = usize>) -> Option<(usize, usize)> {
        draws
            .enumerate()
            .map(|(idx, draw)| {
                self.mark(draw);
                if self.is_won() {
                    Some((idx, self.base_score() * draw))
                } else {
                    None
                }
            })
            .flatten()
            .next()
    }
}

fn score_sort_key(a: &Option<(usize, usize)>, b: &Option<(usize, usize)>) -> Ordering {
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some((aturns, ascore)), Some((bturns, bscore))) => {
            if aturns < bturns {
                Ordering::Greater
            } else if aturns > bturns {
                Ordering::Less
            } else {
                ascore.cmp(bscore)
            }
        }
    }
}

fn iter_scores<P: AsRef<Path>>(input: P) -> Result<impl Iterator<Item = Option<(usize, usize)>>> {
    let mut blocks = stream_file_blocks(input).unwrap();
    let draws = get_draws(&blocks.next().unwrap()[0]);
    Ok(blocks
        .map(|b| BingoField::try_from(b).unwrap())
        .map(move |mut b| b.score_with_draws(draws.iter().copied())))
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(iter_scores(input)?
        .max_by(score_sort_key)
        .flatten()
        .unwrap()
        .1)
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(iter_scores(input)?
        .min_by(score_sort_key)
        .flatten()
        .unwrap()
        .1)
}

const INPUT: &str = "input/day04.txt";

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    use aoc2021::{stream_file_blocks, test_helpers::create_line_file};
    use indoc::indoc;
    use tempfile::TempDir;

    use crate::get_draws;

    fn example_file() -> (TempDir, impl AsRef<Path>) {
        create_line_file(
            [indoc! {"
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
            8  2 23  4 24
            21  9 14 16  7
            6 10  3 18  5
            1 12 20 15 19

            3 15  0  2 22
            9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
            2  0 12  3  7
            "}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_read_draws() {
        let (dir, file) = example_file();
        let first = &stream_file_blocks(file).unwrap().next().unwrap()[0];
        assert_eq!(
            get_draws(first),
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
        drop(dir);
    }

    #[test]
    fn test_read_bingo() {
        let (dir, file) = example_file();
        let bingo_str = stream_file_blocks(file).unwrap().skip(1).next().unwrap();
        let bingo = BingoField::try_from(bingo_str).unwrap();
        assert_eq!(
            bingo.content.iter().map(|(n, _)| *n).collect::<Vec<_>>(),
            vec![
                22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20,
                15, 19
            ]
        );
        drop(dir);
    }

    #[test]
    fn test_score_bingo() {
        let (dir, file) = example_file();
        let mut blocks = stream_file_blocks(file).unwrap();
        let draws = get_draws(&blocks.next().unwrap()[0]);
        let bingo_str = blocks.skip(2).next().unwrap();
        let mut bingo = BingoField::try_from(bingo_str).unwrap();
        assert_eq!(bingo.score_with_draws(draws.into_iter()), Some((11, 4512)));
        drop(dir);
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 4512);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 1924);
        drop(dir);
    }
}
