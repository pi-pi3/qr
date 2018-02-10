use std::mem;

use num_traits::{AsPrimitive, Signed};

use point::Point2;

pub trait Coord<T> {
    fn point(&self) -> Point2<T>;

    fn barycentric(&self) -> Option<&[T]> {
        None
    }
}

pub trait Drawable<T, C: Coord<T>>: IntoIterator<Item = C> {
    fn vertices(&self) -> usize;
}

pub trait Renderer<T: Signed + AsPrimitive<usize>> {
    type Pixel;
    type Attr: Into<Self::Pixel>;
    type Error;

    fn put_pixel(&mut self, p: Point2<usize>, px: Self::Pixel);
    fn swap(&mut self);
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    #[allow(unused_variables)]
    fn get_attr(&self, attr: usize) -> Option<Self::Attr> {
        None
    }

    #[allow(unused_variables)]
    fn set_attr(&mut self, attr: usize, px: Self::Attr) {}

    fn draw<C: Coord<T>, D: Drawable<T, C>, I: Iterator<Item = D>>(
        &mut self,
        mesh: I,
    ) -> Result<(usize, usize, usize), Self::Error> {
        let width = self.width();
        let height = self.height();
        let result = mesh.fold((0, 0, 0), |(shapes, verts, frags), drawable| {
            let verts = verts + drawable.vertices();
            let frags = frags
                + drawable
                    .into_iter()
                    .filter_map(|c| {
                        let (x, y) = c.point();
                        if x.is_positive() && x.as_() < width && y.is_positive() && y.as_() < height
                        {
                            Some((x.as_(), y.as_()))
                        } else {
                            None
                        }
                    })
                    .fold(0, |frags, p| {
                        self.get_attr(0).map(|attr| self.put_pixel(p, attr.into()));
                        frags + 1
                    });

            (shapes + 1, verts, frags)
        });

        Ok(result)
    }
}

pub struct SimpleRenderer<Px> {
    color: Option<Px>,
    width: usize,
    height: usize,
    front: Vec<Px>,
    back: Vec<Px>,
}

impl<Px> SimpleRenderer<Px> {
    pub fn buffer(&self) -> &[Px] {
        &self.front
    }
}

impl<Px: Default + Clone> SimpleRenderer<Px> {
    pub fn new(width: usize, height: usize) -> Self {
        SimpleRenderer {
            color: None,
            width,
            height,
            front: vec![Px::default(); width * height],
            back: vec![Px::default(); width * height],
        }
    }
}

impl<Px: Clone> Renderer<f64> for SimpleRenderer<Px> {
    type Pixel = Px;
    type Attr = Self::Pixel;
    type Error = ();

    fn put_pixel(&mut self, p: Point2<usize>, px: Self::Pixel) {
        self.back[p.1 * self.width + p.0] = px;
    }

    fn swap(&mut self) {
        mem::swap(&mut self.front, &mut self.back)
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get_attr(&self, _attr: usize) -> Option<Self::Attr> {
        self.color.clone()
    }

    fn set_attr(&mut self, _attr: usize, color: Self::Attr) {
        self.color = Some(color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        use std::iter;
        use tri::Triangle;

        let mut renderer = SimpleRenderer::<u8>::new(16, 16);

        let v1 = (0.0, 0.0);
        let v2 = (100.0, 0.0);
        let v3 = (0.0, 100.0);

        let triangle = Triangle::with_points([v1, v2, v3]);

        renderer.set_attr(0, 1);
        assert!(renderer.draw(iter::once(triangle)).is_ok());

        renderer.swap();
        assert_eq!(renderer.buffer(), [1_u8; 16 * 16].as_ref());
    }
}
