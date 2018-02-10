use num_traits::{AsPrimitive, Float};
use line_drawing::{FloatNum, SignedNum};

use renderer::{Coord, Drawable};
use point::{self, Point, Point2};
use line::{self, Line};
use rect::{self, Rectangle};
use tri::{self, Triangle};

#[derive(Clone, Copy, Debug)]
pub enum Shape<T> {
    Point(Point<T>),
    Line(Line<T>),
    Rect(Rectangle<T>),
    Tri(Triangle<T>),
}

impl<T: Float + FloatNum + SignedNum + AsPrimitive<i64> + Copy + 'static> Drawable<T, Point2<T>>
    for Shape<T>
where
    i64: AsPrimitive<T>,
{
    fn vertices(&self) -> usize {
        match *self {
            Shape::Point(ref point) => point.vertices(),
            Shape::Line(ref line) => line.vertices(),
            Shape::Rect(ref rect) => rect.vertices(),
            Shape::Tri(ref tri) => tri.vertices(),
        }
    }
}

impl<T> From<Point<T>> for Shape<T> {
    fn from(point: Point<T>) -> Self {
        Shape::Point(point)
    }
}

impl<T> From<Line<T>> for Shape<T> {
    fn from(line: Line<T>) -> Self {
        Shape::Line(line)
    }
}

impl<T> From<Rectangle<T>> for Shape<T> {
    fn from(rect: Rectangle<T>) -> Self {
        Shape::Rect(rect)
    }
}

impl<T> From<Triangle<T>> for Shape<T> {
    fn from(tri: Triangle<T>) -> Self {
        Shape::Tri(tri)
    }
}

impl<T: Float + FloatNum + SignedNum + AsPrimitive<i64> + Copy + 'static> IntoIterator for Shape<T>
where
    i64: AsPrimitive<T>,
{
    type Item = Point2<T>;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Shape::Point(point) => IntoIter::Point(point.into_iter()),
            Shape::Line(line) => IntoIter::Line(line.into_iter()),
            Shape::Rect(rect) => IntoIter::Rect(rect.into_iter()),
            Shape::Tri(tri) => IntoIter::Tri(tri.into_iter()),
        }
    }
}

#[derive(Debug)]
pub enum IntoIter<T: Float + FloatNum + SignedNum + Copy + 'static>
where
    i64: AsPrimitive<T>,
{
    Point(point::IntoIter<T>),
    Line(line::IntoIter<T>),
    Rect(rect::IntoIter<T>),
    Tri(tri::IntoIter<T>),
}

impl<T: Float + FloatNum + SignedNum + Copy + 'static> Iterator for IntoIter<T>
where
    i64: AsPrimitive<T>,
{
    type Item = Point2<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            IntoIter::Point(ref mut point) => point.next(),
            IntoIter::Line(ref mut line) => line.next().map(|c| c.point()),
            IntoIter::Rect(ref mut rect) => rect.next(),
            IntoIter::Tri(ref mut tri) => tri.next().map(|c| c.point()),
        }
    }
}
