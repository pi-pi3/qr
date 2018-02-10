use std::ops::Range;
use std::marker::PhantomData;

use num_traits::AsPrimitive;

use renderer::Drawable;
use point::Point2;

#[derive(Clone, Copy, Debug)]
pub struct Rectangle<T> {
    x0: T,
    x1: T,
    y0: T,
    y1: T,
}

impl<T> Rectangle<T> {
    #[inline(always)]
    pub fn new(x0: T, x1: T, y0: T, y1: T) -> Self {
        Rectangle { x0, x1, y0, y1 }
    }
}

impl<T: Copy + AsPrimitive<i64> + 'static> Drawable<T, Point2<T>> for Rectangle<T>
where
    i64: AsPrimitive<T>,
{
    #[inline(always)]
    fn vertices(&self) -> usize {
        4
    }
}

impl<T: Copy + AsPrimitive<i64> + 'static> IntoIterator for Rectangle<T>
where
    i64: AsPrimitive<T>,
{
    type Item = Point2<T>;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let x0 = self.x0.as_();
        let x1 = self.x1.as_();
        let y0 = self.y0.as_();
        let y1 = self.y1.as_();

        let width = x0..x1;
        let mut height = y0..y1;
        let x = width.clone();
        let y = height.next();
        let _phantom = PhantomData;

        IntoIter {
            width,
            height,
            x,
            y,
            _phantom,
        }
    }
}

#[derive(Debug)]
pub struct IntoIter<T> {
    x: Range<i64>,
    y: Option<i64>,
    width: Range<i64>,
    height: Range<i64>,
    _phantom: PhantomData<T>,
}

impl<T: Copy + 'static> Iterator for IntoIter<T>
where
    i64: AsPrimitive<T>,
{
    type Item = Point2<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x.next().or_else(|| {
            self.height
                .next()
                .and_then(|y| {
                    self.y = Some(y);
                    self.x = self.width.clone();
                    self.x.next()
                })
                .or_else(|| {
                    self.y = None;
                    None
                })
        });

        x.and_then(|x| self.y.map(|y| (x.as_(), y.as_())))
    }
}

#[cfg(test)]
mod tests {
    use super::Rectangle;

    #[test]
    fn rect() {
        assert_eq!(
            Rectangle::new(0, 2, 0, 2).into_iter().collect::<Vec<_>>(),
            [(0, 0), (1, 0), (0, 1), (1, 1),]
        )
    }
}
