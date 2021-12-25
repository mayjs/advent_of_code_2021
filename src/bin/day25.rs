use anyhow::Result;
use aoc2021::{field2d::Field2D, stream_items_from_file};
use itertools::Itertools;
use std::path::Path;

type SeaCucumberField = Field2D<Option<SeaCucumber>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SeaCucumber {
    East,
    South
}

fn parse_input(input: impl Iterator<Item=String>) -> SeaCucumberField {
    Field2D::parse(input, |line| {
        line.chars().map(|c| match c {
            'v' => Some(SeaCucumber::South),
            '>' => Some(SeaCucumber::East),
            '.' => None,
            _ => panic!("Invalid input")
        }).collect_vec()
    }).unwrap()
}

fn step(old: &SeaCucumberField) -> SeaCucumberField {
    let mut res = SeaCucumberField::new_empty(old.width(), old.height());
    // Start with eastward cucumbers
    for x in 0..old.width() {
        for y in 0..old.height() {
            if old[(x,y)] == Some(SeaCucumber::East) {
                let next_x = (x+1) % old.width();
                if old[(next_x,y)].is_none() {
                    res[(next_x,y)] = old[(x,y)];
                } else {
                    res[(x,y)] = old[(x,y)];
                }
            }
        }
    }

    // Southwards cucumbers are more complicated; they need to check eastwards cucumbers from the new state and southward ones from the old state
    for x in 0..old.width() {
        for y in 0..old.height() {
            if old[(x,y)] == Some(SeaCucumber::South) {
                let next_y = (y+1) % old.height();
                if old[(x,next_y)] != Some(SeaCucumber::South) && res[(x, next_y)].is_none() {
                    res[(x,next_y)] = old[(x,y)];
                }else {
                    res[(x,y)] = old[(x,y)];
                }
            }
        }
    }
    res
}

fn find_fixed_point<T, F>(init: T, mut conversion: F) -> (T, usize) 
where F: FnMut(&T) -> T,
      T: PartialEq {
    let mut cur = init;
    let mut counter = 0;
    loop {
        let next = conversion(&cur);
        counter += 1;
        if next == cur {
            return (next, counter)
        }
        cur = next;
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let lines = stream_items_from_file(input)?;
    let field = parse_input(lines);
    let (_, iterations) = find_fixed_point(field, step);
    Ok(iterations)
}

fn part2<P: AsRef<Path>>(_input: P) -> Result<usize> {
    Ok(0)
}

const INPUT: &str = "input/day25.txt";

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
                v...>>.vv>
                .vv>>.vv..
                >>.>v>...v
                >>v>>.>.v.
                v>v.vv.v..
                >.>>..v...
                .vv..>.>v.
                v.v..>>v.v
                ....v..v.>"}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 58);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 0);
        drop(dir);
    }
}
