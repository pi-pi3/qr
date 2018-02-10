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

#[cfg(test)]
mod tests {
    use rand;
    use image::{ImageBuffer, Pixel, Rgb};

    use super::Triangle;

    #[test]
    fn tri() {
        assert_eq!(
            Triangle::with_points([(0.0, 0.0), (3.0, 0.0), (0.0, 3.0)])
                .into_iter()
                .map(|((x, y), p)| (
                    (x as i64, y as i64),
                    (
                        (p[0] * 3.0) as i64,
                        (p[1] * 3.0) as i64,
                        (p[2] * 3.0) as i64
                    )
                ))
                .collect::<Vec<_>>(),
            [
                ((0, 0), (3, 0, 0)),
                ((1, 0), (2, 1, 0)),
                ((2, 0), (1, 2, 0)),
                ((0, 1), (2, 0, 1)),
                ((1, 1), (1, 1, 1)),
                ((2, 1), (0, 2, 1)),
                ((0, 2), (1, 0, 2)),
                ((1, 2), (0, 1, 2)),
            ]
        )
    }

    #[test]
    fn random_tri() {
        const BPP: usize = 24;
        const WIDTH: usize = 1024;
        const HEIGHT: usize = 1024;
        const SIZE: usize = WIDTH * HEIGHT * BPP / 8;

        let buffer = vec![0_u8; SIZE];
        let image = ImageBuffer::from_raw(WIDTH as _, HEIGHT as _, buffer);
        let mut image = image.unwrap();

        let x0 = rand::random::<f64>() * WIDTH as f64;
        let x1 = rand::random::<f64>() * WIDTH as f64;
        let x2 = rand::random::<f64>() * WIDTH as f64;

        let y0 = rand::random::<f64>() * HEIGHT as f64;
        let y1 = rand::random::<f64>() * HEIGHT as f64;
        let y2 = rand::random::<f64>() * HEIGHT as f64;

        let r0 = rand::random::<f64>() * 255.0;
        let r1 = rand::random::<f64>() * 255.0;
        let r2 = rand::random::<f64>() * 255.0;

        let g0 = rand::random::<f64>() * 255.0;
        let g1 = rand::random::<f64>() * 255.0;
        let g2 = rand::random::<f64>() * 255.0;

        let b0 = rand::random::<f64>() * 255.0;
        let b1 = rand::random::<f64>() * 255.0;
        let b2 = rand::random::<f64>() * 255.0;

        let v1 = (x0, y0);
        let v2 = (x1, y1);
        let v3 = (x2, y2);

        let triangle = Triangle::with_points([v1, v2, v3]);

        triangle
            .into_iter()
            .map(|((x, y), b)| ((x as u32, y as u32), b))
            .for_each(|((x, y), p)| {
                let r = p[0] * r0 + p[1] * r1 + p[2] * r2;
                let g = p[0] * g0 + p[1] * g1 + p[2] * g2;
                let b = p[0] * b0 + p[1] * b1 + p[2] * b2;
                image.put_pixel(x, y, Rgb::from_channels(r as u8, g as u8, b as u8, 255));
            });

        image.save("test1.png").expect("couldn't save image");
    }
}
