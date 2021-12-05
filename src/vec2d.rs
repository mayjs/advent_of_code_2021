use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fmt::Display,
    num::ParseIntError,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};
use thiserror::Error;

#[derive(Debug, Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Display for Vec2D<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub type IVec2D = Vec2D<isize>;
pub type UVec2D = Vec2D<usize>;

impl<T> Vec2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T, O> Add<Vec2D<O>> for Vec2D<T>
where
    T: Add<O>,
{
    type Output = Vec2D<T::Output>;

    fn add(self, rhs: Vec2D<O>) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Vec2D<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T, O> Sub<Vec2D<O>> for Vec2D<T>
where
    T: Sub<O>,
{
    type Output = Vec2D<T::Output>;

    fn sub(self, rhs: Vec2D<O>) -> Self::Output {
        Vec2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> SubAssign for Vec2D<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Debug, Error)]
pub enum NumVecParsingError<E> {
    #[error("missing value in input")]
    MissingValue,
    #[error("Invalid integer")]
    ParseNumberError(#[from] E),
}

impl FromStr for UVec2D {
    type Err = NumVecParsingError<ParseIntError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+").unwrap();
        }
        let values: Vec<usize> = RE
            .find_iter(s)
            .take(2)
            .map(|s| s.as_str().parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(UVec2D::new(
            *values.get(0).ok_or(NumVecParsingError::MissingValue)?,
            *values.get(1).ok_or(NumVecParsingError::MissingValue)?,
        ))
    }
}
