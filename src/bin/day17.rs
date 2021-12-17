use anyhow::anyhow;
use anyhow::Result;
use aoc2021::stream_items_from_file;
use itertools::Itertools;
use regex::Regex;
use std::path::Path;

#[derive(Debug)]
struct TargetArea {
    x_area: (i32, i32),
    y_area: (i32, i32),
}

fn parse_input(input: &str) -> Result<TargetArea> {
    // Don't bother checking the fluff around the numbers, just grab the numbers and go
    let re = Regex::new(r"[\d\-]+").unwrap();
    let numbers = re
        .find_iter(input)
        .map(|number| number.as_str().parse())
        .collect::<Result<Vec<_>, _>>()?;
    let nt: (i32, i32, i32, i32) = numbers
        .into_iter()
        .collect_tuple()
        .ok_or(anyhow!("Not enough items"))?;

    Ok(TargetArea {
        x_area: (nt.0, nt.1),
        y_area: (nt.2, nt.3),
    })
}

trait VelocityLogic {
    fn step_velocity(vel: i32) -> i32;
}

struct YVelocityLogic();

impl VelocityLogic for YVelocityLogic {
    fn step_velocity(vel: i32) -> i32 {
        vel - 1
    }
}

struct XVelocityLogic();

impl VelocityLogic for XVelocityLogic {
    fn step_velocity(vel: i32) -> i32 {
        vel - vel.signum()
    }
}

fn check_area_hit<L: VelocityLogic>(target_range: &(i32, i32), mut velocity: i32) -> bool {
    let mut pos = 0;
    let init_cmp = (pos.cmp(&target_range.0), pos.cmp(&target_range.1));

    loop {
        let cmp = (pos.cmp(&target_range.0), pos.cmp(&target_range.1));
        if cmp.0 != cmp.1 {
            return true;
        } else if cmp != init_cmp {
            return false;
        } else {
            pos += velocity;
            let new_velocity = L::step_velocity(velocity);
            if new_velocity == velocity && new_velocity == 0 {
                return false;
            }
            velocity = new_velocity;
        }
    }
}

fn find_max_velocity_y(target_range: &(i32, i32)) -> i32 {
    // Using this velocity, we will have target_range.0 velocity on our 0-crossing, allowing us to do a single step to the end of the target range from there
    -target_range.0 - 1
}

fn get_y_range(target_range: &(i32, i32)) -> Vec<i32> {
    let min = target_range.0; // Fastest downwards shot we can do is immediately reaching the target region
    let max = find_max_velocity_y(target_range);
    (min..=max)
        .filter(|&vel| check_area_hit::<YVelocityLogic>(target_range, vel))
        .collect()
}

// Find an approximate minimal value for the x velocity that will get us to the given target value (Using the inverse of the Gauss formula)
fn find_x_velocity_approx(target: i32) -> i32 {
    (((2 * target) as f64 + 0.25).sqrt() - 0.5).floor() as i32
}

fn get_x_range(target_range: &(i32, i32)) -> Vec<i32> {
    let min = find_x_velocity_approx(target_range.0);
    let max = target_range.1; // Fastest we can do is a single step to the end of the target range

    // Filter for values that actually end up hitting the target range
    (min..=max)
        .filter(|&vel| check_area_hit::<XVelocityLogic>(target_range, vel))
        .collect()
}

fn find_max_height(velocity: i32) -> i32 {
    if velocity < 0 {
        0
    } else {
        (velocity * (velocity + 1)) / 2
    }
}

fn check_hit(mut velocity: (i32, i32), target: &TargetArea) -> bool {
    let mut pos = (0, 0);
    loop {
        if pos.0 > target.x_area.1 || pos.1 < target.y_area.0 {
            return false;
        }
        // We haven't overshot the outer bounds of our target yet; did we cross the lower bounds?
        if pos.0 >= target.x_area.0 && pos.1 <= target.y_area.1 {
            return true;
        }
        pos.0 += velocity.0;
        pos.1 += velocity.1;
        velocity = (
            XVelocityLogic::step_velocity(velocity.0),
            YVelocityLogic::step_velocity(velocity.1),
        );
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<i32> {
    let target = parse_input(
        &stream_items_from_file::<_, String>(input)?
            .next()
            .ok_or(anyhow!("No input"))?,
    )?;
    let max_v = find_max_velocity_y(&target.y_area);
    Ok(find_max_height(max_v))
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let target = parse_input(
        &stream_items_from_file::<_, String>(input)?
            .next()
            .ok_or(anyhow!("No input"))?,
    )?;
    let xrange = get_x_range(&target.x_area);
    let yrange = get_y_range(&target.y_area);

    Ok(xrange
        .iter()
        .map(|&xvel| {
            let target = &target;
            yrange
                .iter()
                .filter(move |&&yvel| check_hit((xvel, yvel), &target))
                .map(move |&yvel| (xvel, yvel))
        })
        .flatten()
        .count())
}

const INPUT: &str = "input/day17.txt";

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use aoc2021::test_helpers::create_line_file;
    use tempfile::TempDir;

    use super::*;

    fn example_file() -> (TempDir, impl AsRef<Path>) {
        create_line_file(["target area: x=20..30, y=-10..-5"].iter(), None)
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 45);
        drop(dir);
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file();
        assert_eq!(part2(file).unwrap(), 112);
        drop(dir);
    }
}
