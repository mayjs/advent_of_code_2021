use anyhow::Result;
use aoc2021::{field2d::Field2D, stream_items_from_file};
use std::path::Path;

fn grow<T: Clone + Default>(input: &Field2D<T>, amount: usize) -> Field2D<T> {
    let mut res = Field2D::new_empty(input.width() + 2 * amount, input.height() + 2 * amount);
    for x in 0..input.width() {
        for y in 0..input.height() {
            res[(x + amount, y + amount)] = input[(x, y)].clone();
        }
    }
    res
}

fn translate_string_repr(input: String) -> Vec<bool> {
    input
        .chars()
        .map(|c| match c {
            '#' => true,
            _ => false,
        })
        .collect()
}

fn read_input_field(input: impl Iterator<Item = String>) -> Field2D<bool> {
    let field = Field2D::parse(input, translate_string_repr).unwrap();
    grow(&field, 2)
}

fn step_field(old_field: &Field2D<bool>, replacement_table: &Vec<bool>) -> Field2D<bool> {
    let mut new_field = Field2D::new_empty(old_field.width() + 4, old_field.height() + 4);
    for x in 1..old_field.width() - 1 {
        for y in 1..old_field.height() - 1 {
            let lookup = (0..3)
                .map(|ny| (0..3).map(move |nx| old_field[(x - 1 + nx, y - 1 + ny)]))
                .flatten()
                .fold(0, |sum, bit| (sum * 2) + if bit { 1 } else { 0 });
            new_field[(x + 2, y + 2)] = replacement_table[lookup];
        }
    }

    new_field
}

fn visualize_field(field: &Field2D<bool>) {
    for y in 0..field.height() {
        for x in 0..field.width() {
            print!("{}", if field[(x, y)] { '#' } else { '.' })
        }
        println!("");
    }
}

fn simulate(mut field: Field2D<bool>, replacement_table: Vec<bool>, steps: usize) -> Field2D<bool> {
    for i in 0..steps {
        field = step_field(&field, &replacement_table);
        // This is a hack to get proper simulations of the infinite fields even if index 0 of the replacement table is not `false`.
        // This still requires that index 255 in the replacement is `false`!
        // Basically, the step function will always create a new 2-wide ring of `false` values around the entire image, 
        // and this ring must be completely lit up if we are on an even step...
        if i %2 == 0 && replacement_table[0] { 
            let w =field.width();
            let h = field.height();
            for x in 0..w {
                for y in 0..3 {
                    field[(x,y)] = true;
                    field[(x,h - 1 - y)] = true;
                }
            }
            for x in 0..3 {
                for y in 0..h {
                    field[(x,y)] = true;
                    field[(w-1-x,y)] = true;
                }
            }
        }
    }
    field
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut lines = stream_items_from_file::<_, String>(input)?;
    let replacement_table = translate_string_repr(lines.next().unwrap());
    lines.next();
    let mut field = read_input_field(lines);

    field = simulate(field, replacement_table, 2);

    visualize_field(&field);

    let lit_pixels = field.into_iter().filter(|&x| x).count();

    Ok(lit_pixels)
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut lines = stream_items_from_file::<_, String>(input)?;
    let replacement_table = translate_string_repr(lines.next().unwrap());
    lines.next();
    let mut field = read_input_field(lines);

    field = simulate(field, replacement_table, 50);

    visualize_field(&field);

    let lit_pixels = field.into_iter().filter(|&x| x).count();

    Ok(lit_pixels)
}

const INPUT: &str = "input/day20.txt";

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
            ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
            
            #..#.
            #....
            ##..#
            ..#..
            ..###"}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 35);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 3351);
        drop(dir);
    }
}
