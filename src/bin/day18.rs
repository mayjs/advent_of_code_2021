use anyhow::anyhow;
use anyhow::{bail, Result};
use aoc2021::stream_items_from_file;
use itertools::Itertools;
use std::fmt::Debug;
use std::{cell::RefCell, iter::Peekable, path::Path, rc::Rc, str::FromStr};

// Walkable SnailFishExpr tree

#[derive(Debug)]
enum SnailFishExpr {
    Constant(usize),
    Pair(Rc<RefCell<SnailFishExpr>>, Rc<RefCell<SnailFishExpr>>),
}

impl SnailFishExpr {
    fn const_value(&self) -> Option<usize> {
        match self {
            SnailFishExpr::Constant(v) => Some(*v),
            SnailFishExpr::Pair(_, _) => None,
        }
    }

    fn pair(left: SnailFishExpr, right: SnailFishExpr) -> Self {
        Self::Pair(Rc::new(RefCell::new(left)), Rc::new(RefCell::new(right)))
    }

    fn simple_pair(left: usize, right: usize) -> Self {
        Self::pair(
            SnailFishExpr::Constant(left),
            SnailFishExpr::Constant(right),
        )
    }

    fn magnitude(&self) -> usize {
        match self {
            SnailFishExpr::Constant(v) => *v,
            SnailFishExpr::Pair(left, right) => {
                3 * left.borrow().magnitude() + 2 * right.borrow().magnitude()
            }
        }
    }

    fn deep_copy(&self) -> Self {
        match self {
            SnailFishExpr::Constant(v) => Self::Constant(*v),
            SnailFishExpr::Pair(left, right) => Self::pair(left.borrow().deep_copy(), right.borrow().deep_copy()),
        }
    }
}

#[derive(Debug)]
struct SnailFishCursorImpl {
    current: Rc<RefCell<SnailFishExpr>>,
    parent: Option<Rc<SnailFishCursorImpl>>,
}

trait SnailFishCursor
where
    Self: Sized,
{
    fn left(&self) -> Option<Self>;
    fn right(&self) -> Option<Self>;
    fn depth(&self) -> usize;
    fn parent(&self) -> Option<Self>;
    fn get_const_value(&self) -> Option<usize>;
    fn set_value(&self, value: usize);
    fn replace_node(&self, node: SnailFishExpr);
    fn is_value_pair(&self) -> bool;
    fn same(&self, other: &Self) -> bool;
}

trait AsCursor {
    fn as_cursor(&self) -> SnailFishCursorImpl;
}

impl AsCursor for Rc<RefCell<SnailFishExpr>> {
    fn as_cursor(&self) -> SnailFishCursorImpl {
        SnailFishCursorImpl {
            parent: None,
            current: self.clone(),
        }
    }
}

fn descend(
    cursor: &Rc<SnailFishCursorImpl>,
    child: &Rc<RefCell<SnailFishExpr>>,
) -> Rc<SnailFishCursorImpl> {
    Rc::new(SnailFishCursorImpl {
        current: child.clone(),
        parent: Some(cursor.clone()),
    })
}

impl SnailFishCursor for Rc<SnailFishCursorImpl> {
    fn left(&self) -> Option<Self> {
        match &*self.current.borrow() {
            SnailFishExpr::Constant(_) => None,
            SnailFishExpr::Pair(left, _) => Some(descend(self, left)),
        }
    }

    fn right(&self) -> Option<Self> {
        match &*self.current.borrow() {
            SnailFishExpr::Constant(_) => None,
            SnailFishExpr::Pair(_, right) => Some(descend(self, right)),
        }
    }

    fn depth(&self) -> usize {
        1 + self.parent.as_ref().map(|p| p.depth()).unwrap_or(0)
    }

    fn parent(&self) -> Option<Self> {
        self.parent.clone()
    }

    fn get_const_value(&self) -> Option<usize> {
        self.current.as_ref().borrow().const_value()
    }

    fn set_value(&self, value: usize) {
        self.current.replace(SnailFishExpr::Constant(value));
    }

    fn replace_node(&self, node: SnailFishExpr) {
        self.current.replace(node);
    }

    fn is_value_pair(&self) -> bool {
        self.left()
            .and_then(|node| node.get_const_value().map(|_| true))
            .unwrap_or(false)
            || self
                .right()
                .and_then(|node| node.get_const_value().map(|_| true))
                .unwrap_or(false)
    }

    fn same(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.current, &other.current)
    }
}

fn find_left_neighbor_const<T: SnailFishCursor>(mut cursor: T) -> Option<T> {
    loop {
        let new_cursor = cursor.parent()?;
        if let Some(left) = new_cursor.left() {
            if !left.same(&cursor) {
                cursor = left;
                while let Some(right) = cursor.right() {
                    cursor = right;
                }
                return Some(cursor);
            }
        }
        cursor = new_cursor;
    }
}

fn find_right_neighbor_const<T: SnailFishCursor>(mut cursor: T) -> Option<T> {
    loop {
        let new_cursor = cursor.parent()?;
        if let Some(right) = new_cursor.right() {
            if !right.same(&cursor) {
                cursor = right;
                while let Some(left) = cursor.left() {
                    cursor = left;
                }
                return Some(cursor);
            }
        }
        cursor = new_cursor;
    }
}

fn explode(cursor: impl SnailFishCursor + Clone + Debug) {
    let left_value = cursor
        .left()
        .expect("Explode must not be called on leafs")
        .get_const_value()
        .expect("Explode must only be called on simple pairs");
    let right_value = cursor
        .right()
        .expect("Explode must not be called on leafs")
        .get_const_value()
        .expect("Explode must only be called on simple pairs");

    find_left_neighbor_const(cursor.clone()).map(|node| {
        let old_value = node
            .get_const_value()
            .expect("Find left neighbor must return a constant");
        node.set_value(old_value + left_value);
    });
    find_right_neighbor_const(cursor.clone()).map(|node| {
        let old_value = node
            .get_const_value()
            .expect("Find left neighbor must return a constant");
        node.set_value(old_value + right_value);
    });

    cursor.set_value(0);
}

fn split(cursor: impl SnailFishCursor + Clone) {
    let value = cursor
        .get_const_value()
        .expect("Can only split const value");
    cursor.replace_node(SnailFishExpr::simple_pair(value / 2, (value + 1) / 2));
}

fn reduce_step_explode(root: impl SnailFishCursor + Clone + Debug) -> bool {
    if root.depth() == 5 && root.is_value_pair() {
        explode(root);
        true
    } else {
        root.left().map(reduce_step_explode).unwrap_or(false)
            || root.right().map(reduce_step_explode).unwrap_or(false)
    }
}

fn reduce_step_split(root: impl SnailFishCursor + Clone + Debug) -> bool {
    if root.get_const_value().map(|v| v >= 10).unwrap_or_default() {
        split(root);
        true
    } else {
        root.left().map(reduce_step_split).unwrap_or(false)
            || root.right().map(reduce_step_split).unwrap_or(false)
    }
}

fn reduce(root: impl SnailFishCursor + Clone + Debug) {
    loop {
        if !(reduce_step_explode(root.clone()) || reduce_step_split(root.clone())) {
            return;
        }
    }
}

// Snailfish Expr parser
fn consume(iter: &mut impl Iterator<Item = char>, expected: char) -> Result<()> {
    let next = iter
        .next()
        .ok_or(anyhow!("Unexpected end of input, wanted: '{}'", expected))?;
    if next != expected {
        bail!("Unexpected input (Got '{}', expected '{}')", next, expected);
    }
    Ok(())
}

fn parse_snailfish(iter: &mut Peekable<impl Iterator<Item = char>>) -> Result<SnailFishExpr> {
    match iter.peek().ok_or(anyhow!("Empty input!"))? {
        '[' => {
            iter.next();
            let left = parse_snailfish(iter)?;
            consume(iter, ',')?;
            let right = parse_snailfish(iter)?;
            consume(iter, ']')?;
            Ok(SnailFishExpr::Pair(
                Rc::new(RefCell::new(left)),
                Rc::new(RefCell::new(right)),
            ))
        }
        c if c.is_digit(10) => {
            let mut number = String::new();
            while let Some(digit) = iter.next_if(|c| c.is_digit(10)) {
                number.push(digit);
            }
            Ok(SnailFishExpr::Constant(number.parse()?))
        }
        c @ _ => bail!("Unexpected char '{}'", c),
    }
}

impl FromStr for SnailFishExpr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_snailfish(&mut s.chars().peekable())
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut expressions = stream_items_from_file::<_, SnailFishExpr>(input)?;
    let mut sum = Rc::new(RefCell::new(expressions.next().unwrap()));
    reduce(Rc::new(sum.as_cursor()));
    for expression in expressions {
        let expr = Rc::new(RefCell::new(expression));
        reduce(Rc::new(expr.as_cursor()));

        sum = Rc::new(RefCell::new(SnailFishExpr::Pair(sum, expr)));
        reduce(Rc::new(sum.as_cursor()));
    }
    let magnitude = sum.borrow().magnitude();
    Ok(magnitude)
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let expressions = stream_items_from_file::<_, SnailFishExpr>(input)?.map(|e| Rc::new(RefCell::new(e))).collect_vec();
    // Assuming that every number needs to be reduced first
    expressions.iter().for_each(|ex| {
        reduce(Rc::new(ex.as_cursor()));
    });
    let max = expressions.iter().map(|a| {
        // Just assume that adding the same number twice is also allowed...
        expressions.iter().map(|b| {
            let sum = Rc::new(RefCell::new(SnailFishExpr::pair(a.borrow().deep_copy(), b.borrow().deep_copy())));
            reduce(Rc::new(sum.as_cursor()));
            let magnitude = sum.borrow().magnitude();
            magnitude
        }).max().unwrap()
    }).max().unwrap();
    Ok(max)
}

const INPUT: &str = "input/day18.txt";

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
                [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
                [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
                [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
                [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
                [7,[5,[[3,8],[1,4]]]]
                [[2,[2,2]],[8,[8,1]]]
                [2,9]
                [1,[[[9,3],9],[[9,0],[0,7]]]]
                [[[5,[7,4]],7],1]
                [[[[4,2],2],6],[8,7]]"}]
            .iter(),
            None,
        )
    }

    fn example_file1() -> (TempDir, impl AsRef<Path>) {
        create_line_file(
            [indoc! {"
                [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
                [[[5,[2,8]],4],[5,[[9,9],0]]]
                [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
                [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
                [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
                [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
                [[[[5,4],[7,7]],8],[[8,3],8]]
                [[9,3],[[9,9],[6,[4,9]]]]
                [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
                [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"}]
            .iter(),
            None,
        )
    }

    #[test]
    fn test_part1() {
        let (dir, file) = example_file();
        assert_eq!(part1(file).unwrap(), 3488);
        drop(dir);
        let (dir, file) = example_file1();
        assert_eq!(part1(file).unwrap(), 4140);
        drop(dir)
    }

    #[test]
    fn test_part2() {
        let (dir, file) = example_file1();
        assert_eq!(part2(file).unwrap(), 3993);
        drop(dir);
    }
}
