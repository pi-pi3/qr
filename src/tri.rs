use num_traits::{AsPrimitive, Float};

use renderer::{Coord, Drawable};
use point::Point2;
use rect::{self, Rectangle};

pub type Coordinate<T> = (Point2<T>, [T; 3]);

impl<T: Copy> Coord<T> for Coordinate<T> {
    #[inline(always)]
    fn point(&self) -> Point2<T> {
        self.0
    }

    #[inline(always)]
    fn barycentric(&self) -> Option<&[T]> {
        Some(&self.1)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle<T> {
    points: [Point2<T>; 3],
}

impl<T: Float + AsPrimitive<i64> + Copy + 'static> Drawable<T, Coordinate<T>> for Triangle<T>
where
    i64: AsPrimitive<T>,
{
    #[inline(always)]
    fn vertices(&self) -> usize {
        3
    }
}

impl<T: Float + AsPrimitive<i64>> Triangle<T> {
    #[inline(always)]
    pub fn with_points(points: [Point2<T>; 3]) -> Triangle<T> {
        Triangle { points }
    }

    #[inline]
    pub fn det(&self) -> T {
        let (x1, y1) = self.points[0];
        let (x2, y2) = self.points[1];
        let (x3, y3) = self.points[2];

        (y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3)
    }
}

impl<T: Float + AsPrimitive<i64> + Copy + 'static> IntoIterator for Triangle<T>
where
    i64: AsPrimitive<T>,
{
    type Item = Coordinate<T>;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let det = self.det();
        let points = self.points;
        let min_x = points[0].0.min(points[1].0).min(points[2].0);
        let max_x = points[0].0.max(points[1].0).max(points[2].0);
        let min_y = points[0].1.min(points[1].1).min(points[2].1);
        let max_y = points[0].1.max(points[1].1).max(points[2].1);
        let rect = Rectangle::new(min_x, max_x, min_y, max_y).into_iter();

        IntoIter { det, rect, points }
    }
}

#[derive(Debug)]
pub struct IntoIter<T> {
    det: T,
    rect: rect::IntoIter<T>,
    points: [Point2<T>; 3],
}

impl<T: Float + Copy + 'static> Iterator for IntoIter<T>
where
    i64: AsPrimitive<T>,
{
    type Item = Coordinate<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.rect.next() {
                None => break None,
                Some((x, y)) => {
                    let (x1, y1) = self.points[0];
                    let (x2, y2) = self.points[1];
                    let (x3, y3) = self.points[2];
                    let p1 = ((y2 - y3) * (x - x3) + (x3 - x2) * (y - y3)) / self.det;
                    let p2 = ((y3 - y1) * (x - x3) + (x1 - x3) * (y - y3)) / self.det;

                    if p1 >= T::zero() && p2 >= T::zero() && p1 + p2 <= T::one() {
                        let p3 = T::one() - p1 - p2;
                        break Some(((x, y), [p1, p2, p3]));
                    }
                }
            }
        }
    }
}
