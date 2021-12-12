use anyhow::Result;
use aoc2021::stream_items_from_file;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    path::Path,
    str::FromStr,
};

#[derive(Debug)]
struct Graph<T> {
    node_lookup: HashMap<T, usize>,
    adjacencies: Vec<HashSet<usize>>,
}

impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self {
            node_lookup: Default::default(),
            adjacencies: Default::default(),
        }
    }
}

impl<T> Graph<T>
where
    T: Hash + Eq,
{
    fn insert_node(&mut self, node: T) -> usize {
        match self.node_lookup.get(&node) {
            Some(&v) => v,
            None => {
                let v = self.node_lookup.len();
                self.node_lookup.insert(node, v);
                self.adjacencies.push(Default::default());
                v
            }
        }
    }

    fn connect(&mut self, a: T, b: T) -> (usize, usize) {
        let av = self.insert_node(a);
        let bv = self.insert_node(b);
        self.adjacencies[av].insert(bv);
        self.adjacencies[bv].insert(av);
        (av, bv)
    }

    fn _get_node_value(&self, index: usize) -> Option<&T> {
        self.node_lookup
            .iter()
            .find(|(_, &idx)| idx == index)
            .map(|(n, _)| n)
    }

    fn get_node_index(&self, node: &T) -> Option<usize> {
        self.node_lookup.get(node).copied()
    }

    fn get_neighbors(&self, node: usize) -> Option<&HashSet<usize>> {
        self.adjacencies.get(node)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Cave {
    SmallCave(String),
    BigCave(String),
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or_default()
        {
            Ok(Self::BigCave(s.to_string()))
        } else {
            Ok(Self::SmallCave(s.to_string()))
        }
    }
}

impl Cave {
    fn is_small(&self) -> bool {
        match self {
            Cave::SmallCave(_) => true,
            Cave::BigCave(_) => false,
        }
    }
}

#[derive(Debug, Default)]
struct CaveSystem(Graph<Cave>, HashSet<usize>);

impl CaveSystem {
    fn parse(input: impl Iterator<Item = String>) -> Self {
        let mut connections = Graph::<Cave>::default();
        let mut small_caves = HashSet::<usize>::new();

        for line in input {
            let (left, right) = line
                .split('-')
                .map(|name| Cave::from_str(name).unwrap())
                .collect_tuple()
                .unwrap();
            let (left_small, right_small) = (left.is_small(), right.is_small());
            let (left_idx, right_idx) = connections.connect(left, right);
            if left_small {
                small_caves.insert(left_idx);
            }
            if right_small {
                small_caves.insert(right_idx);
            }
        }

        CaveSystem(connections, small_caves)
    }

    fn dfs_search(
        &self,
        cur_path: &mut Vec<usize>,
        visited_small_nodes: &mut HashSet<usize>,
        target: usize,
        double: bool,
        start: usize,
    ) -> usize {
        let cur = *cur_path.last().unwrap();
        let mut paths = 0;
        for neighbor in self.0.get_neighbors(cur).unwrap() {
            if *neighbor == target {
                paths += 1;
            } else {
                let second_small = visited_small_nodes.contains(neighbor);
                if !second_small || (!double && *neighbor != start) {
                    if self.1.contains(neighbor) {
                        visited_small_nodes.insert(*neighbor);
                    }
                    cur_path.push(*neighbor);
                    paths += self.dfs_search(
                        cur_path,
                        visited_small_nodes,
                        target,
                        double || second_small,
                        start,
                    );
                    cur_path.pop();
                    if !second_small {
                        visited_small_nodes.remove(neighbor);
                    }
                }
            }
        }

        return paths;
    }

    fn find_all_paths(&self, from: &Cave, to: &Cave, allow_double: bool) -> usize {
        let start = self.0.get_node_index(from).unwrap();
        let end = self.0.get_node_index(to).unwrap();
        let mut start_path = vec![start];
        let mut visited_small_nodes = HashSet::new();
        visited_small_nodes.insert(start);

        self.dfs_search(
            &mut start_path,
            &mut visited_small_nodes,
            end,
            !allow_double,
            start,
        )
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let cave_system = CaveSystem::parse(stream_items_from_file(input)?);
    Ok(cave_system.find_all_paths(
        &Cave::SmallCave("start".to_string()),
        &Cave::SmallCave("end".to_string()),
        false,
    ))
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let cave_system = CaveSystem::parse(stream_items_from_file(input)?);
    Ok(cave_system.find_all_paths(
        &Cave::SmallCave("start".to_string()),
        &Cave::SmallCave("end".to_string()),
        true,
    ))
}

const INPUT: &str = "input/day12.txt";

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

    fn example_file1() -> (TempDir, impl AsRef<Path>) {
        create_line_file(
            [indoc! {"
                start-A
                start-b
                A-c
                A-b
                b-d
                A-end
                b-end"}]
            .iter(),
            None,
        )
    }

    fn example_file2() -> (TempDir, impl AsRef<Path>) {
        create_line_file(
            [indoc! {"
                dc-end
                HN-start
                start-kj
                dc-start
                dc-HN
                LN-dc
                HN-end
                kj-sa
                kj-HN
                kj-dc"}]
            .iter(),
            None,
        )
    }

    fn example_file3() -> (TempDir, impl AsRef<Path>) {
        create_line_file(
            [indoc! {"
                fs-end
                he-DX
                fs-he
                start-DX
                pj-DX
                end-zg
                zg-sl
                zg-pj
                pj-he
                RW-he
                fs-DX
                pj-RW
                zg-RW
                start-pj
                he-WI
                zg-he
                pj-fs
                start-RW"}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file1();
        assert_eq!(part1(file).unwrap(), 10);
        drop(dir);
        let (dir, file) = example_file2();
        assert_eq!(part1(file).unwrap(), 19);
        drop(dir);
        let (dir, file) = example_file3();
        assert_eq!(part1(file).unwrap(), 226);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file1();
        assert_eq!(part2(file).unwrap(), 36);
        drop(dir);
        let (dir, file) = example_file2();
        assert_eq!(part2(file).unwrap(), 103);
        drop(dir);
        let (dir, file) = example_file3();
        assert_eq!(part2(file).unwrap(), 3509);
        drop(dir);
    }
}
