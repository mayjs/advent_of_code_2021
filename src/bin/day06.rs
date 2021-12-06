use anyhow::Result;
use aoc2021::stream_items_from_file;
use std::path::Path;

type Population = [usize; 9];

trait PopulationSim {
    fn step(&mut self);
    fn population_size(&self) -> usize;
}

fn parse_lines(input: impl Iterator<Item = String>) -> Population {
    let mut output = Population::default();
    input.for_each(|l| {
        l.split(',')
            .map(|s| s.parse::<usize>().expect("Invalid input"))
            .for_each(|individual: usize| output[individual] += 1)
    });
    output
}

impl PopulationSim for Population {
    fn step(&mut self) {
        let spawns = self[0];
        for age in 1..=8 {
            self[age-1] = self[age];
        }
        self[6] += spawns;
        self[8] = spawns;
    }

    fn population_size(&self) -> usize {
        self.iter().sum()
    }
}

fn run_simulation(population: &mut impl PopulationSim, steps: usize) -> usize{
    for _ in 0..steps {
        population.step();
    }
    population.population_size()
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut population = parse_lines(stream_items_from_file(input)?);
    Ok(run_simulation(&mut population, 80))
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut population = parse_lines(stream_items_from_file(input)?);
    Ok(run_simulation(&mut population, 256))
}

const INPUT: &str = "input/day06.txt";

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use aoc2021::{test_helpers::create_line_file, stream_items_from_file};
    use tempfile::TempDir;

    use super::*;

    fn example_file() -> (TempDir, impl AsRef<Path>) {
        create_line_file(["3,4,3,1,2"].iter(), None)
    }

    #[test]
    fn test_simulation() {
        let (dir, file) = example_file();
        let mut population = parse_lines(stream_items_from_file::<_,String>(file).unwrap());
        assert_eq!(run_simulation(&mut population, 18), 26);
        drop(dir);
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 5934);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 26984457539);
        drop(dir);
    }
}
