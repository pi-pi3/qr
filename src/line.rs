use std::fmt::{self, Debug};

use line_drawing::{FloatNum, Midpoint, SignedNum};

use renderer::{Coord, Drawable};
use point::Point2;

pub type Coordinate<T> = (Point2<T>, [T; 2]);

impl<T: Copy> Coord<T> for Coordinate<T> {
    fn point(&self) -> Point2<T> {
        self.0
    }

    fn barycentric(&self) -> Option<&[T]> {
        Some(&self.1)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line<T> {
    start: Point2<T>,
    end: Point2<T>,
}

impl<T: FloatNum + SignedNum> Drawable<T, Coordinate<T>> for Line<T> {
    fn vertices(&self) -> usize {
        2
    }
}

impl<T: FloatNum + SignedNum> IntoIterator for Line<T> {
    type Item = Coordinate<T>;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let start = self.start;
        let dx = self.end.0 - self.start.0;
        let dy = self.end.1 - self.start.1;
        let len = (dx * dx + dy * dy).sqrt();
        let len_recip = len.recip();
        let inner = Midpoint::new(self.start, self.end);

        IntoIter {
            start,
            inner,
            len_recip,
        }
    }
}

pub struct IntoIter<T: FloatNum + SignedNum> {
    start: Point2<T>,
    inner: Midpoint<T, T>,
    len_recip: T,
}

impl<T: FloatNum + SignedNum> Iterator for IntoIter<T> {
    type Item = Coordinate<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(x, y)| {
            let dx = x - self.start.0;
            let dy = y - self.start.1;
            let dist = (dx * dx + dy * dy).sqrt();
            let f = dist * self.len_recip;
            ((x, y), [f, T::one() - f])
        })
    }
}

impl<T: FloatNum + SignedNum + Debug> Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IntoIter")
            .field("start", &self.start)
            .field("len_recip", &self.len_recip)
            .field("inner", &"...")
            .finish()
    }
}
