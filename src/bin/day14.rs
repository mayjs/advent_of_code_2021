use anyhow::Result;
use aoc2021::stream_items_from_file;
use itertools::Itertools;
use std::{collections::HashMap, path::Path};

type ElementCounts = HashMap<char, usize>;
type ElementPairCounts = HashMap<(char, char), usize>;
type PairInsertionRules = HashMap<(char, char), char>;

fn parse_input(
    mut input: impl Iterator<Item = String>,
) -> (ElementCounts, ElementPairCounts, PairInsertionRules) {
    let polymer_template = input.next().unwrap();

    let element_counts =
        polymer_template
            .chars()
            .fold(ElementCounts::new(), |mut counts, element| {
                *counts.entry(element).or_insert(0) += 1;
                counts
            });
    let element_pair_counts = polymer_template.chars().tuple_windows().fold(
        ElementPairCounts::new(),
        |mut counts, pair| {
            *counts.entry(pair).or_insert(0) += 1;
            counts
        },
    );

    let rules: PairInsertionRules = input
        .filter_map(|line| {
            line.split(" -> ")
                .map(|part| part.to_string())
                .collect_tuple::<(_, _)>()
        })
        .map(|(pair, produce)| {
            (
                pair.chars().collect_tuple().unwrap(),
                produce.chars().next().unwrap(),
            )
        })
        .collect();

    (element_counts, element_pair_counts, rules)
}

fn execute_rules(
    counts: &mut ElementCounts,
    pairs: ElementPairCounts,
    rules: &PairInsertionRules,
) -> ElementPairCounts {
    let mut new_pairs = ElementPairCounts::new();
    for (pair, count) in pairs.into_iter() {
        if rules.contains_key(&pair) {
            let insert = rules[&pair];
            *counts.entry(insert).or_insert(0) += count;
            *new_pairs.entry((pair.0, insert)).or_insert(0) += count;
            *new_pairs.entry((insert, pair.1)).or_insert(0) += count;
        } else {
            new_pairs.insert(pair, count);
        }
    }

    new_pairs
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let (mut counts, mut pairs, rules) = parse_input(stream_items_from_file(input)?);
    for _ in 0..10 {
        pairs = execute_rules(&mut counts, pairs, &rules);
    }

    let (min, max) = counts.values().minmax().into_option().unwrap();
    Ok(max - min)
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let (mut counts, mut pairs, rules) = parse_input(stream_items_from_file(input)?);
    for _ in 0..40 {
        pairs = execute_rules(&mut counts, pairs, &rules);
    }

    let (min, max) = counts.values().minmax().into_option().unwrap();
    Ok(max - min)
}

const INPUT: &str = "input/day14.txt";

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
                NNCB

                CH -> B
                HH -> N
                CB -> H
                NH -> C
                HB -> C
                HC -> B
                HN -> C
                NN -> C
                BH -> H
                NC -> B
                NB -> B
                BN -> B
                BB -> N
                BC -> B
                CC -> N
                CN -> C
            "}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 1588);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 2188189693529);
        drop(dir);
    }
}
