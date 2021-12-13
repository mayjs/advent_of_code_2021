use anyhow::Result;
use aoc2021::{stream_items_from_file, vec2d::Vec2D};
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashSet, path::Path};

type Dots = HashSet<Vec2D<usize>>;
type Folds = Vec<Vec2D<usize>>;

fn parse_input(input: impl Iterator<Item = String>) -> Result<(Dots, Folds)> {
    let fold_re = Regex::new(r"^fold along (\w)=(\d+)$").expect("Regex syntax failure");

    let mut dots = Dots::new();
    let mut folds = Folds::new();

    for line in input.filter(|l| l.len() > 0) {
        if let Some(m) = fold_re.captures(&line) {
            let fold_pos = m.get(2).unwrap().as_str().parse::<usize>()?;
            let fold = match m.get(1).unwrap().as_str() {
                "x" => Vec2D::new(fold_pos, 0),
                "y" => Vec2D::new(0, fold_pos),
                _ => anyhow::bail!("Invalid fold descriptor {}", line),
            };
            folds.push(fold);
        } else {
            dots.insert(line.parse::<_>()?);
        }
    }

    Ok((dots, folds))
}

fn is_after_fold(dot: &Vec2D<usize>, fold: &Vec2D<usize>) -> bool {
    (fold.x > 0 && dot.x >= fold.x) || (fold.y > 0 && dot.y >= fold.y)
}

fn is_on_fold(dot: &Vec2D<usize>, fold: &Vec2D<usize>) -> bool {
    (fold.x > 0 && dot.x == fold.x) || (fold.y > 0 && dot.y == fold.y)
}

fn execute_fold(mut dots: Dots, fold: &Vec2D<usize>) -> Dots {
    let mut new_dots = Dots::new();
    for mut dot in dots.drain() {
        if is_after_fold(&dot, fold) {
            if fold.x != 0 {
                dot.x = 2 * fold.x - dot.x;
            }
            if fold.y != 0 {
                dot.y = 2 * fold.y - dot.y;
            }
            new_dots.insert(dot);
        } else if !is_on_fold(&dot, fold) {
            new_dots.insert(dot);
        }
    }
    new_dots
}

fn render_dots(dots: &Dots) -> String {
    let width = dots.iter().map(|dot| dot.x).max().unwrap() + 1;
    let height = dots.iter().map(|dot| dot.y).max().unwrap() + 1;

    let mut result = vec![vec![' '; width]; height];

    for dot in dots {
        result[dot.y][dot.x] = 'x';
    }

    result
        .iter()
        .map(|l| String::from_iter(l.iter()))
        .join("\n")
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let (dots, folds) = parse_input(stream_items_from_file(input)?)?;
    let dots = execute_fold(dots, folds.first().unwrap());
    Ok(dots.len())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<String> {
    let (dots, folds) = parse_input(stream_items_from_file(input)?)?;
    let folded = folds
        .into_iter()
        .fold(dots, |dots, fold| execute_fold(dots, &fold));

    Ok(render_dots(&folded))
}

const INPUT: &str = "input/day13.txt";

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2:\n{}", part2(INPUT)?);
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
                6,10
                0,14
                9,10
                0,3
                10,4
                4,11
                6,0
                6,12
                4,1
                0,13
                10,12
                3,4
                3,0
                8,4
                1,10
                2,14
                8,10
                9,0
                
                fold along y=7
                fold along x=5"}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 17);
        drop(dir);
    }

    // No test for part 2, don't want to bother with it
}
