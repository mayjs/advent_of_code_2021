use std::{
    iter::repeat_with,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Field2D<T> {
    values: Vec<T>,
    width: usize,
}

impl<T> Field2D<T>
where
    T: Default,
{
    pub fn add_row(&mut self) {
        self.values
            .extend(repeat_with(Default::default).take(self.width()))
    }

    pub fn new_empty(width: usize, height: usize) -> Self {
        let mut res = Field2D {
            values: Vec::with_capacity(width * height),
            width,
        };
        for _ in 0..height {
            res.add_row();
        }
        res
    }
}

impl<T> Field2D<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.values.len() / self.width
    }

    pub fn len(&self) -> usize {
        self.width() * self.height()
    }

    pub fn neighbors(&self, x: usize, y: usize) -> NeighborIter {
        NeighborIter {
            field_size: (self.width(), self.height()),
            pos: (x, y),
            diag: false,
            state: NeighborIterState::default(),
        }
    }

    pub fn neighbors_diag(&self, x: usize, y: usize) -> NeighborIter {
        NeighborIter {
            field_size: (self.width(), self.height()),
            pos: (x, y),
            diag: true,
            state: NeighborIterState::default(),
        }
    }

    pub fn parse<R, F, I>(mut rows: impl Iterator<Item = R>, mut parser: F) -> Option<Self>
    where
        F: FnMut(R) -> I,
        I: IntoIterator<Item = T>,
    {
        let mut res = Vec::new();
        if let Some(first) = rows.next() {
            res.extend(parser(first));
            let width = res.len();
            for row in rows {
                res.extend(parser(row));
            }
            Some(Self {values: res, width})
        } else {
            None
        }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
        self.values.iter_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.values.iter()
    }
}

impl<T> Index<(usize, usize)> for Field2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        assert!(x < self.width());
        assert!(y < self.height());
        &self.values[x + y * self.width()]
    }
}

impl<T> IndexMut<(usize, usize)> for Field2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        assert!(x < self.width());
        assert!(y < self.height());
        let width = self.width();
        &mut self.values[x + y * width]
    }
}

impl<T> IntoIterator for Field2D<T> {
    type Item=T;

    type IntoIter=<Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NeighborIterState {
    Right,
    Down,
    Left,
    Up,
    UpLeft,
    UpRight,
    DownRight,
    DownLeft,
    Done,
}

impl Default for NeighborIterState {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Debug, Clone)]
pub struct NeighborIter {
    field_size: (usize, usize),
    pos: (usize, usize),
    diag: bool,
    state: NeighborIterState,
}

impl NeighborIter {
    fn cur(&self) -> Option<(usize, usize)> {
        use NeighborIterState::*;
        let dx: i32 = match self.state {
            Right | UpRight | DownRight => 1,
            Left | UpLeft | DownLeft => -1,
            Up | Down => 0,
            Done => return None,
        };
        if dx == -1 && self.pos.0 == 0 {
            return None;
        }
        if dx == 1 && self.pos.0 == self.field_size.0 - 1 {
            return None;
        }

        let dy: i32 = match self.state {
            Up | UpLeft | UpRight => -1,
            Down | DownLeft | DownRight => 1,
            Left | Right => 0,
            Done => return None,
        };
        if dy == -1 && self.pos.1 == 0 {
            return None;
        }
        if dy == 1 && self.pos.1 == self.field_size.1 - 1 {
            return None;
        }

        // TODO: The casting here is not nice
        Some((
            (self.pos.0 as i32 + dx) as usize,
            (self.pos.1 as i32 + dy) as usize,
        ))
    }

    fn advance(&mut self) {
        use NeighborIterState::*;
        self.state = match self.state {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => {
                if self.diag {
                    UpLeft
                } else {
                    Done
                }
            }
            UpLeft => UpRight,
            UpRight => DownRight,
            DownRight => DownLeft,
            DownLeft => Done,
            Done => Done,
        }
    }
}

impl Iterator for NeighborIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.state != NeighborIterState::Done {
            if let Some(v) = self.cur() {
                self.advance();
                return Some(v);
            } else {
                self.advance();
            }
        }
        None
    }
}
