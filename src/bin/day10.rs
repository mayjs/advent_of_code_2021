use anyhow::Result;
use aoc2021::stream_items_from_file;
use itertools::{Either, Itertools};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ElementType {
    Paren,
    Bracket,
    Angle,
    Curly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SyntaxError {
    found: ElementType,
    expected: Option<ElementType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenKind {
    Opening,
    Closing,
}

struct Token {
    typ: ElementType,
    kind: TokenKind,
}

impl Token {
    fn new(typ: ElementType, kind: TokenKind) -> Self {
        Self { typ, kind }
    }
}

fn tokenize(line: impl AsRef<str>) -> Vec<Token> {
    line.as_ref()
        .chars()
        .map(|c| match c {
            '[' => Token::new(ElementType::Bracket, TokenKind::Opening),
            ']' => Token::new(ElementType::Bracket, TokenKind::Closing),
            '(' => Token::new(ElementType::Paren, TokenKind::Opening),
            ')' => Token::new(ElementType::Paren, TokenKind::Closing),
            '<' => Token::new(ElementType::Angle, TokenKind::Opening),
            '>' => Token::new(ElementType::Angle, TokenKind::Closing),
            '{' => Token::new(ElementType::Curly, TokenKind::Opening),
            '}' => Token::new(ElementType::Curly, TokenKind::Closing),
            c => panic!("Invalid char {}", c),
        })
        .collect()
}

fn search_syntax_error(line: impl AsRef<str>) -> Either<Vec<ElementType>, SyntaxError> {
    let mut stack = Vec::new();
    let tokens = tokenize(line);

    for token in tokens {
        match token.kind {
            TokenKind::Opening => {
                stack.push(token.typ);
            }
            TokenKind::Closing => {
                let expected = stack.pop();
                if expected != Some(token.typ) {
                    return Either::Right(SyntaxError {
                        found: token.typ,
                        expected: expected,
                    });
                }
            }
        }
    }

    Either::Left(stack)
}

fn get_all_syntax_errors(input: impl Iterator<Item = String>) -> impl Iterator<Item = SyntaxError> {
    input.map(search_syntax_error).filter_map(Either::right)
}

fn get_all_incomplete_lines(
    input: impl Iterator<Item = String>,
) -> impl Iterator<Item = Vec<ElementType>> {
    input.map(search_syntax_error).filter_map(Either::left)
}

fn score_completion(missing: Vec<ElementType>) -> u64 {
    missing
        .iter()
        .rev()
        .map(|typ| match typ {
            ElementType::Paren => 1,
            ElementType::Bracket => 2,
            ElementType::Curly => 3,
            ElementType::Angle => 4,
        })
        .fold(0, |acc, v| (acc * 5) + v)
}

fn score_error(error: &SyntaxError) -> u32 {
    use ElementType::*;
    match error.found {
        Paren => 3,
        Bracket => 57,
        Curly => 1197,
        Angle => 25137,
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<u32> {
    Ok(get_all_syntax_errors(stream_items_from_file(input)?)
        .map(|e| score_error(&e))
        .sum())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<u64> {
    let mut scores = get_all_incomplete_lines(stream_items_from_file(input)?)
        .map(score_completion)
        .collect_vec();
    scores.sort();
    Ok(scores[scores.len() / 2])
}

const INPUT: &str = "input/day10.txt";

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
    use itertools::Itertools;
    use tempfile::TempDir;

    use super::*;

    fn example_file() -> (TempDir, impl AsRef<Path>) {
        create_line_file(
            [indoc! {"
                [({(<(())[]>[[{[]{<()<>>
                [(()[<>])]({[<{<<[]>>(
                {([(<{}[<>[]}>{[]{[(<()>
                (((({<>}<{<{<>}{[]{[]{}
                [[<[([]))<([[{}[[()]]]
                [{[{({}]{}}([{[{{{}}([]
                {<[[]]>}<{[{[{[]{()[[[]
                [<(<(<(<{}))><([]([]()
                <{([([[(<>()){}]>(<<{{
                <{([{{}}[<[[[<>{}]]]>[]]"}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_syntax_checker() {
        let (dir, file) = example_file();
        let errors = get_all_syntax_errors(stream_items_from_file(file).unwrap()).collect_vec();
        use ElementType::*;

        assert_eq!(
            errors,
            vec![
                SyntaxError {
                    found: Curly,
                    expected: Some(Bracket)
                },
                SyntaxError {
                    found: Paren,
                    expected: Some(Bracket)
                },
                SyntaxError {
                    found: Bracket,
                    expected: Some(Paren)
                },
                SyntaxError {
                    found: Paren,
                    expected: Some(Angle)
                },
                SyntaxError {
                    found: Angle,
                    expected: Some(Bracket)
                },
            ]
        );
        drop(dir);
    }

    #[test]
    fn test_completion() {
        let (dir, file) = example_file();
        let scores = get_all_incomplete_lines(stream_items_from_file(file).unwrap())
            .map(score_completion)
            .collect_vec();
        assert_eq!(scores, vec![288957, 5566, 1480781, 995444, 294]);
        drop(dir);
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 26397);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 288957);
        drop(dir);
    }
}
