use std::iter::{self, Once};
use std::ops::{Deref, DerefMut};

use renderer::{Coord, Drawable};

/// 2D euclidean point.
pub type Point2<T> = (T, T);

impl<T: Copy> Coord<T> for Point2<T> {
    #[inline(always)]
    fn point(&self) -> Point2<T> {
        *self
    }
}

/// Primitive drawable point.
#[derive(Clone, Copy, Debug)]
pub struct Point<T>(pub Point2<T>);

impl<T: Copy> Drawable<T, Point2<T>> for Point<T> {
    #[inline(always)]
    fn vertices(&self) -> usize {
        1
    }
}

impl<T> Deref for Point<T> {
    type Target = Point2<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Point<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Yo dawg, I heard you liked points, so I made the point containing a point
/// into an iterator that returns that point exactly once.
pub type IntoIter<T> = Once<Point2<T>>;

impl<T> IntoIterator for Point<T> {
    type Item = Point2<T>;
    type IntoIter = IntoIter<T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self.0)
    }
}
