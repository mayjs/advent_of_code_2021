use anyhow::anyhow;
use anyhow::Result;
use aoc2021::stream_items_from_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, path::Path};

trait Die {
    fn roll(&mut self) -> usize;
}

struct PracticeDie {
    counter: usize,
    limit: usize,
}

impl PracticeDie {
    fn new(limit: usize) -> Self {
        PracticeDie { counter: 0, limit }
    }
}

impl Die for PracticeDie {
    fn roll(&mut self) -> usize {
        self.counter += 1;
        let res = self.counter;
        self.counter %= self.limit;
        res
    }
}

fn game(
    mut die: impl Die,
    score_limit: usize,
    starting_positions: (usize, usize),
) -> (usize, usize) {
    let mut player1_pos = starting_positions.0;
    let mut player2_pos = starting_positions.1;
    let mut player1_score = 0;
    let mut player2_score = 0;
    let mut throws = 0;
    loop {
        let fields: usize = (0..3).map(|_| die.roll()).sum();
        player1_pos = ((player1_pos + fields - 1) % 10) + 1;
        player1_score += player1_pos;
        throws += 3;
        if player1_score >= score_limit {
            return (player2_score, throws);
        }

        let fields: usize = (0..3).map(|_| die.roll()).sum();
        player2_pos = ((player2_pos + fields - 1) % 10) + 1;
        player2_score += player2_pos;
        throws += 3;
        if player2_score >= score_limit {
            return (player1_score, throws);
        }
    }
}

fn extract_starting_position(line: &str) -> Result<usize> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[\d]+$").unwrap();
    }

    let nmatch = RE.find(line).ok_or(anyhow!("No number in line"))?;
    Ok(nmatch.as_str().parse()?)
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let starting_positions: Vec<usize> = stream_items_from_file::<_, String>(input)?
        .map(|line| extract_starting_position(&line))
        .collect::<Result<_>>()?;
    let die = PracticeDie::new(100);
    let (loosing_score, throws) = game(die, 1000, (starting_positions[0], starting_positions[1]));
    Ok(loosing_score * throws)
}

fn get_dice_combinations(sides: usize) -> HashMap<usize, usize> {
    let mut res = HashMap::new();
    for one in 1..=sides {
        for two in 1..=sides {
            for three in 1..=sides {
                *res.entry(one + two + three).or_insert(0) += 1;
            }
        }
    }
    res
}

lazy_static! {
    static ref DIRAC_DIE_COMBINATIONS: HashMap<usize, usize> = get_dice_combinations(3);
}

fn dirac_game(
    p1move: bool,
    p1pos: usize,
    p2pos: usize,
    p1score: usize,
    p2score: usize,
) -> (usize, usize) {
    let moving_player_pos = if p1move { p1pos } else { p2pos };
    let moving_player_score = if p1move { p1score } else { p2score };

    let mut result = (0, 0);
    for (steps, options) in DIRAC_DIE_COMBINATIONS.iter() {
        let new_pos = ((moving_player_pos + steps - 1) % 10) + 1;
        let new_score = moving_player_score + new_pos;
        if new_score >= 21 {
            if p1move {
                result.0 += options;
            } else {
                result.1 += options;
            }
        } else {
            let sub = if p1move {
                dirac_game(false, new_pos, p2pos, new_score, p2score)
            } else {
                dirac_game(true, p1pos, new_pos, p1score, new_score)
            };
            result.0 += options * sub.0;
            result.1 += options * sub.1;
        }
    }
    result
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let starting_positions: Vec<usize> = stream_items_from_file::<_, String>(input)?
        .map(|line| extract_starting_position(&line))
        .collect::<Result<_>>()?;
    let results = dirac_game(true, starting_positions[0], starting_positions[1], 0, 0);
    Ok([results.0, results.1].into_iter().max().unwrap())
}

const INPUT: &str = "input/day21.txt";

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
                Player 1 starting position: 4
                Player 2 starting position: 8"}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 739785);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 444356092776315);
        drop(dir);
    }
}
