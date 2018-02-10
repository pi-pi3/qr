use std::iter::{self, Once};
use std::ops::{Deref, DerefMut};

use renderer::{Coord, Drawable};

pub type Point2<T> = (T, T);

impl<T: Copy> Coord<T> for Point2<T> {
    fn point(&self) -> Point2<T> {
        *self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point<T>(Point2<T>);

impl<T: Copy> Drawable<T, Point2<T>> for Point<T> {
    fn vertices(&self) -> usize {
        1
    }
}

impl<T> Deref for Point<T> {
    type Target = Point2<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Point<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub type IntoIter<T> = Once<Point2<T>>;

impl<T> IntoIterator for Point<T> {
    type Item = Point2<T>;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once(self.0)
    }
}
