use anyhow::Result;
use aoc2021::{field2d::Field2D, stream_items_from_file};
use itertools::Itertools;
use std::{collections::HashSet, path::Path};

#[derive(Debug, Clone)]
struct OctopusEnergies(Field2D<u32>);

impl OctopusEnergies {
    fn parse(input: impl Iterator<Item = String>) -> Self {
        OctopusEnergies(
            Field2D::parse(input, |line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("Invalid input char"))
                    .collect_vec()
                    .into_iter()
            })
            .unwrap(),
        )
    }

    fn step(&mut self) -> usize {
        // Step 1: Increment all energy levels
        self.0.iter_mut().for_each(|v| *v += 1);

        // Step 2: Flash every octopus with energy level > 9
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();
        let flashes = loop {
            let old_flash_state = flashed.len();
            for x in 0..self.0.width() {
                for y in 0..self.0.height() {
                    if self.0[(x, y)] > 9 && !flashed.contains(&(x, y)) {
                        for neighbor in self.0.neighbors_diag(x, y) {
                            self.0[neighbor] += 1;
                        }
                        flashed.insert((x, y));
                    }
                }
            }
            if old_flash_state == flashed.len() {
                break flashed.len();
            }
        };

        // Step 3: Reset all counters
        flashed.into_iter().for_each(|coords| self.0[coords] = 0);
        flashes
    }

    fn simulate(&mut self, nsteps: usize) -> usize {
        (0..nsteps).map(|_| self.step()).sum()
    }

    fn find_sync(&mut self) -> usize {
        let field_size = self.0.len();
        // Run an infinite simulation and stop as soon as all octopuses flash
        std::iter::repeat_with(|| self.step())
            .enumerate()
            .filter(|(_, flashes)| *flashes == field_size)
            .next()
            .unwrap()
            .0
            + 1
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut energies = OctopusEnergies::parse(stream_items_from_file(input)?);
    Ok(energies.simulate(100))
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut energies = OctopusEnergies::parse(stream_items_from_file(input)?);
    Ok(energies.find_sync())
}

const INPUT: &str = "input/day11.txt";

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
                5483143223
                2745854711
                5264556173
                6141336146
                6357385478
                4167524645
                2176841721
                6882881134
                4846848554
                5283751526"}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_short_sim() {
        let (dir, file) = example_file();
        let lines = stream_items_from_file(file).unwrap();
        let mut energies = OctopusEnergies::parse(lines);
        assert_eq!(energies.simulate(10), 204);
        drop(dir);
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 1656);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 195);
        drop(dir);
    }
}
