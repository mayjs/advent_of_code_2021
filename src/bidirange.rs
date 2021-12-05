use std::{cmp::Ordering, ops::AddAssign};

#[derive(Debug, Clone)]
pub struct BidiRange<T, S> {
    end: T,
    step: S,
    cur: T,
    begin: Ordering,
}

impl<T, S> Iterator for BidiRange<T, S>
where
    T: AddAssign<S> + Ord + Copy,
    S: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let cmp = self.cur.cmp(&self.end);
        if cmp != self.begin && cmp != Ordering::Equal {
            None
        } else {
            let out = self.cur;
            self.cur += self.step;
            Some(out)
        }
    }
}

pub fn bidi_range(start: isize, end: isize) -> BidiRange<isize, isize> {
    let step = if start <= end { 1 } else { -1 };
    BidiRange {
        end,
        step,
        cur: start,
        begin: start.cmp(&end),
    }
}
