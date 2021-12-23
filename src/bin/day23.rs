use anyhow::Result;
use aoc2021::stream_items_from_file;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    path::Path,
    rc::Rc,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Token {
    A,
    B,
    C,
    D,
}

impl Token {
    fn specific_cost(&self) -> usize {
        match self {
            Token::A => 1,
            Token::B => 10,
            Token::C => 100,
            Token::D => 1000,
        }
    }

    fn target_room(&self) -> usize {
        match self {
            Token::A => 0,
            Token::B => 1,
            Token::C => 2,
            Token::D => 3,
        }
    }

    fn from_room(room_id: usize) -> Token {
        match room_id {
            0 => Token::A,
            1 => Token::B,
            2 => Token::C,
            3 => Token::D,
            _ => panic!("Invalid Room"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct GameState {
    room_size: usize,
    rooms: [Vec<Token>; 4],
    hallway_spaces: [Option<Token>; 3],
    hallway_storage: [[Option<Token>; 2]; 2],
}

impl GameState {
    fn new_empty(room_size: usize) -> GameState {
        GameState {
            room_size,
            rooms: Default::default(),
            hallway_spaces: Default::default(),
            hallway_storage: Default::default(),
        }
    }

    fn new_finished(room_size: usize) -> GameState {
        let mut empty = GameState::new_empty(room_size);
        for room_id in 0..empty.rooms.len() {
            empty.rooms[room_id] = vec![Token::from_room(room_id); room_size];
        }
        empty
    }

    fn room_token(room_id: usize) -> Token {
        match room_id {
            0 => Token::A,
            1 => Token::B,
            2 => Token::C,
            3 => Token::D,
            _ => panic!("Room ID {} is out of bounds", room_id),
        }
    }

    fn room_exit_cost(&self, room_id: usize) -> usize {
        self.room_size - self.rooms[room_id].len()
    }

    fn room_enter_cost(&self, room_id: usize) -> usize {
        self.room_size - self.rooms[room_id].len()
    }

    fn generate_next_states(&self) -> Vec<(usize, GameState)> {
        let mut states = Vec::new();
        for room_id in 0..4 {
            if self.rooms[room_id]
                .iter()
                .all(|t| t == &GameState::room_token(room_id))
            {
                // This room is either empty or in a properly sorted state, no need to do anything now
                continue;
            }
            if let Some(token) = self.rooms[room_id].last() {
                // First option: Move from any room into the left storage area
                if self.hallway_storage[0][0].is_none()
                    && (0..room_id).all(|step| self.hallway_spaces[step].is_none())
                {
                    let mut new_state = self.clone();
                    new_state.rooms[room_id].pop();
                    new_state.hallway_storage[0][0] = Some(*token);
                    let cost = self.room_exit_cost(room_id) + 1 + 1 + 2 * room_id;
                    states.push((cost * token.specific_cost(), new_state));
                    if self.hallway_storage[0][1].is_none() {
                        // Move to the back if possible
                        let mut new_state = self.clone();
                        new_state.rooms[room_id].pop();
                        new_state.hallway_storage[0][1] = Some(*token);
                        let cost = self.room_exit_cost(room_id) + 1 + 2 + 2 * room_id;
                        states.push((cost * token.specific_cost(), new_state));
                    }
                }
                // Second option: Move from any room into the right storage area
                if self.hallway_storage[1][0].is_none()
                    && (room_id..3).all(|step| self.hallway_spaces[step].is_none())
                {
                    let mut new_state = self.clone();
                    new_state.rooms[room_id].pop();
                    new_state.hallway_storage[1][0] = Some(*token);
                    let cost = self.room_exit_cost(room_id) + 1 + 1 + 2 * (3 - room_id);
                    states.push((cost * token.specific_cost(), new_state));
                    if self.hallway_storage[1][1].is_none() {
                        // Move to the back if possible
                        let mut new_state = self.clone();
                        new_state.rooms[room_id].pop();
                        new_state.hallway_storage[1][1] = Some(*token);
                        let cost = self.room_exit_cost(room_id) + 1 + 2 + 2 * (3 - room_id);
                        states.push((cost * token.specific_cost(), new_state));
                    }
                }
                // Next option: Move into any of the hallway spaces; this requires that all of the spaces before that hallway space are free as well
                for hallway_target in 0..3 {
                    let step_range = if hallway_target < room_id {
                        hallway_target..=room_id - 1
                    } else {
                        room_id..=hallway_target
                    };
                    if step_range
                        .clone()
                        .any(|step| self.hallway_spaces[step].is_some())
                    {
                        // Path is blocked, can't go this way
                        continue;
                    }
                    // All spaces are free, we are good to go
                    let mut new_state = self.clone();
                    new_state.rooms[room_id].pop();
                    new_state.hallway_spaces[hallway_target] = Some(*token);
                    let cost = self.room_exit_cost(room_id) + step_range.count() * 2;
                    states.push((cost * token.specific_cost(), new_state));
                }
            }
        }

        for hallway_space in 0..3 {
            if let Some(token) = &self.hallway_spaces[hallway_space] {
                let target_room = token.target_room();
                if self.rooms[target_room].len() == self.room_size
                    || self.rooms[target_room]
                        .iter()
                        .any(|t| t.target_room() != target_room)
                {
                    // Target room is full or contains other types, can't enter
                    continue;
                }
                let steps = if target_room <= hallway_space {
                    target_room..hallway_space
                } else {
                    hallway_space + 1..target_room
                };
                if steps
                    .clone()
                    .all(|step| self.hallway_spaces[step].is_none())
                {
                    let mut new_state = self.clone();
                    new_state.hallway_spaces[hallway_space].take();
                    new_state.rooms[target_room].push(*token);
                    let cost = 1 + steps.count() * 2 + self.room_enter_cost(target_room);
                    states.push((cost * token.specific_cost(), new_state));
                }
            }
        }

        for (storage, storage_local) in (0..2).cartesian_product(0..2) {
            if let Some(token) = &self.hallway_storage[storage][storage_local] {
                if storage_local == 0 || self.hallway_storage[storage][0].is_none() {
                    let target_room = token.target_room();
                    if self.rooms[target_room].len() == self.room_size
                        || self.rooms[target_room]
                            .iter()
                            .any(|t| t.target_room() != target_room)
                    {
                        // Target room is full or contains other types, can't enter
                        continue;
                    }
                    let steps = if storage == 0 {
                        0..target_room
                    } else {
                        target_room..3
                    };

                    if steps
                        .clone()
                        .all(|step| self.hallway_spaces[step].is_none())
                    {
                        let mut new_state = self.clone();
                        new_state.hallway_storage[storage][storage_local].take();
                        new_state.rooms[target_room].push(*token);
                        let cost = 1
                            + steps.count() * 2
                            + self.room_enter_cost(target_room)
                            + storage_local;
                        states.push((cost * token.specific_cost(), new_state));
                    }
                }
            }
        }
        states
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PathFindEntry {
    state: Rc<GameState>,
    score: usize,
}

impl PartialOrd for PathFindEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for PathFindEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

fn find_minimal_score(start: GameState) -> Option<usize> {
    let mut open_nodes = BinaryHeap::new();
    let mut known_paths = HashMap::new();
    let mut preds: HashMap<Rc<GameState>, (usize, Rc<GameState>)> = HashMap::new();

    let start = Rc::new(start);
    let goal = GameState::new_finished(start.room_size);

    open_nodes.push(Reverse(PathFindEntry {
        score: 0,
        state: start.clone(),
    }));
    known_paths.insert(start.clone(), 0);

    while let Some(Reverse(current)) = open_nodes.pop() {
        let current_score = known_paths[&current.state];
        if *current.state == goal {
            let mut current = (current_score, current.state);
            let mut path = Vec::new();
            while current.1 != start {
                path.push(current.clone());
                current = preds[&current.1].clone();
            }
            path.push(current.clone());
            // for state in path.iter().rev() {
            //     dbg!(state);
            // }

            return Some(current_score);
        }

        let next_states = current.state.generate_next_states();
        for (score, next_state) in next_states {
            let next_state = Rc::new(next_state);
            let cand_score = known_paths[&current.state] + score;
            if known_paths
                .get(&next_state)
                .iter()
                .all(|&&current_best| cand_score < current_best)
            {
                open_nodes.push(Reverse(PathFindEntry {
                    score: cand_score,
                    state: next_state.clone(),
                }));
                known_paths.insert(next_state.clone(), cand_score);
                preds.insert(next_state, (score, current.state.clone()));
            }
        }
    }

    None
}

fn parse_input(lines: &Vec<String>, room_size: usize) -> Result<GameState> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[ABCD]").unwrap();
    }
    let mut state = GameState::new_empty(room_size);
    for line in lines.iter().rev().skip(1).take(4) {
        for (i, ts) in RE.find_iter(line).enumerate() {
            let tok = match ts.as_str() {
                "A" => Token::A,
                "B" => Token::B,
                "C" => Token::C,
                "D" => Token::D,
                _ => panic!("Should never get this token: {}", ts.as_str()),
            };
            state.rooms[i].push(tok);
        }
    }

    Ok(state)
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let lines = stream_items_from_file(input)?.collect();
    let init = parse_input(&lines, 2)?;
    let score = find_minimal_score(init).expect("No path to final state found!");
    Ok(score)
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut lines: Vec<String> = stream_items_from_file(input)?.collect();
    lines.insert(3, "  #D#C#B#A#".to_string());
    lines.insert(4, "  #D#B#A#C#".to_string());
    let init = parse_input(&lines, 4)?;
    let score = find_minimal_score(init).expect("No path to final state found!");
    Ok(score)
}

const INPUT: &str = "input/day23.txt";

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
            #############
            #...........#
            ###B#C#B#D###
              #A#D#C#A#
              #########"}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 12521);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 44169);
        drop(dir);
    }
}
