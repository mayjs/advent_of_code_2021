use anyhow::Result;
use aoc2021::{field2d::Field2D, stream_items_from_file};
use itertools::Itertools;
use std::{path::Path, collections::{BinaryHeap, HashMap}, cmp::Reverse};

type RiskField = Field2D<u32>;

fn parse_risk_field(input: impl Iterator<Item=String>) -> RiskField {
    RiskField::parse(input, |line| {
        line.chars()
            .map(|c| c.to_digit(10).expect("Invalid input char"))
            .collect_vec()
            .into_iter()
    })
    .unwrap()
}

#[derive(Debug, PartialEq, Eq)]
struct PathFindEntry { 
    score: u32,
    node: (usize,usize),
}

impl PartialOrd for PathFindEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.score.partial_cmp(&other.score) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.node.partial_cmp(&other.node)
    }
}

impl Ord for PathFindEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

fn path_find(field: &RiskField) -> Option<u32> {
    // Simple A* path search without path reconstruction
    let mut open_nodes = BinaryHeap::new();
    let mut known_paths = HashMap::<(usize,usize), u32>::new();

    open_nodes.push(Reverse(PathFindEntry {score: 0, node: (0,0)}));
    known_paths.insert((0,0), 0);

    let goal = (field.width() - 1, field.height() - 1);

    while let Some(Reverse(current)) = open_nodes.pop() {
        if current.node == goal {
            return Some(known_paths[&goal]);
        }

        for neighbor in field.neighbors(current.node.0, current.node.1) {
            let cand_score = known_paths[&current.node] + field[neighbor];
            if known_paths.get(&neighbor).map(|&current_best| cand_score < current_best).unwrap_or(true) {
                known_paths.insert(neighbor.clone(), cand_score);
                /* Use a euclidean distance as the heuristic, this works since every move costs at least 1 risk */
                let heuristic = (((goal.0 - neighbor.0).pow(2) + (goal.1 - neighbor.1).pow(2)) as f32).sqrt();
                open_nodes.push(Reverse(PathFindEntry { score: cand_score + heuristic as u32, node: neighbor}));
            }
        }
    }

    None
}

fn part1<P: AsRef<Path>>(input: P) -> Result<u32> {
    let field = parse_risk_field(stream_items_from_file(input)?);
    let min_risk = path_find(&field).unwrap();
    Ok(min_risk)
}

fn quintuple_field(input: &RiskField) -> RiskField {
    let mut new_field = RiskField::new_empty(input.width() * 5, input.height() * 5);
    (0..5).cartesian_product(0..5).for_each(|(field_x,field_y)| {
        let (offset_x, offset_y) = (field_x * input.width(), field_y * input.height()); 
        (0..input.width()).cartesian_product(0..input.height()).for_each(|(ox,oy)| {
            new_field[(offset_x + ox, offset_y + oy)] = (input[(ox,oy)] + field_x as u32 + field_y as u32 - 1) % 9 + 1;
        });
    });

    new_field
}

fn part2<P: AsRef<Path>>(input: P) -> Result<u32> {
    let field = quintuple_field(&parse_risk_field(stream_items_from_file(input)?));
    let min_risk = path_find(&field).unwrap();
    Ok(min_risk)
}

const INPUT: &str = "input/day15.txt";

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
            1163751742
            1381373672
            2136511328
            3694931569
            7463417111
            1319128137
            1359912421
            3125421639
            1293138521
            2311944581"}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 40);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 315);
        drop(dir);
    }
}
