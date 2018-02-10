//! Traits for rendering generic and arbitrary meshes and primitives.

use std::mem;

use num_traits::{AsPrimitive, Signed};

use point::Point2;

/// A trait for types, which can represent a screenspace point and a local
/// barycentric point.
///
/// # Parameters
///
/// - `T` represents the primitive numeric type used in base computation.
pub trait Coord<T> {
    /// Get the euclidean point.
    fn point(&self) -> Point2<T>;

    /// Get the barycentric point.
    ///
    /// # Returns
    ///
    /// - `None` if `Self` doesn't have a barycentric point.
    /// - Some(&[...]) if `Self` has a barycentric point.
    fn barycentric(&self) -> Option<&[T]> {
        None
    }
}

/// A trait for primitive types, which can be drawn
///
/// # Parameters
///
/// - `T` represents the primitive numeric type used in base computation.
/// - `C` represents the euclidean/barycentric coordinate returned by this shapes
/// `IntoIter`.
pub trait Drawable<T, C: Coord<T>>: IntoIterator<Item = C> {
    /// The count of vertices this `Drawable` has
    fn vertices(&self) -> usize;
}

/// A trait for types, which can be used to draw meshes and primitives
///
/// # Parameters
///
/// - `T` represents the primitive numeric type used in base computation.
pub trait Renderer<T: Signed + AsPrimitive<usize>> {
    /// The pixel type that gets drawn to the buffer.
    type Pixel;
    /// The auxilliary attribute type. Can represent i.e. geometry normal or
    /// texture uv. Implements `Into<Self::Pixel>` for debugging purposes.
    type Attr: Into<Self::Pixel>;
    /// The potential error that the `draw` method can return.
    type Error;

    /// Put `px` at coordinate `p`.
    fn put_pixel(&mut self, p: Point2<usize>, px: Self::Pixel);
    /// Swap back and front buffers.
    fn swap(&mut self);
    /// Get the width of the buffer.
    fn width(&self) -> usize;
    /// Get the height of the buffer.
    fn height(&self) -> usize;

    /// Get the n'th attribute.
    /// 
    /// # Returns
    ///
    /// - `Some(Self::Attr)` if `Self` supports auxilliary attributes.
    /// - `None` if `Self` doesn't supports attributes.
    #[allow(unused_variables)]
    fn get_attr(&self, attr: usize) -> Option<Self::Attr> {
        None
    }

    /// Set the n'th attribute.
    #[allow(unused_variables)]
    fn set_attr(&mut self, attr: usize, val: Self::Attr) {}

    /// Draw the `mesh` (i.e. a `Drawable`) with previously set attributes.
    ///
    /// # Parameters
    ///
    /// - `I` represents an `Iterator` over primitives, a.k.a. a mesh.
    /// - `D` represents the type of the primitive returned by `I`.
    /// - `C` represents the `Coordinate` type of the `Drawable`.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` to represent the possibility of
    /// failure. The `Ok` variant contains debug+statistics information.
    ///
    /// - `Ok((shapes, vertices, fragments))` `shapes` is the count of shapes
    /// drawn. `vertices` is the count of vertices of the many `shapes`.
    /// `fragments` is the count of fragments put to the screen.
    /// - `Err(Self::Error)` if something went bad.
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

/// A simple renderer for quick-start and reference `impl`-ementation of the
/// `Renderer` trait. It can draw any mesh and primitive using floating point
/// math and a single color.
///
/// # Example
///
/// ```
/// extern crate qr;
///
/// use std::iter;
///
/// use qr::{Triangle, Renderer, SimpleRenderer};
///
/// const WIDTH: usize = 128;
/// const HEIGHT: usize = 128;
///
/// fn main() {
///     let mut renderer = SimpleRenderer::new(WIDTH, HEIGHT);
///     let triangle = Triangle::with_points([(0.0, 0.0), (100.0, 100.0), (0.0, 100.0)]);
///     let mesh = iter::once(triangle);
///
///     renderer.set_attr(0, (255_u8, 255_u8, 255_u8));
///     if let Ok((s, v, f)) = renderer.draw(mesh) {
///         println!("drawn {} primitives, {} vertices and {} fragments", s, v, f);
///     }
/// }
/// ```
///
/// # Parameters
///
/// - `Px` represents the pixel type, a.k.a. a color.
pub struct SimpleRenderer<Px: Clone> {
    /// The currently selected color, if any.
    color: Option<Px>,
    /// The width of the internal buffers.
    width: usize,
    /// The height of the internal buffers.
    height: usize,
    /// The public "read-only" buffer.
    front: Vec<Px>,
    /// The private "write-only" buffer.
    back: Vec<Px>,
}

impl<Px: Clone> SimpleRenderer<Px> {
    /// Get a reference to the public "read-only" buffer.
    pub fn buffer(&self) -> &[Px] {
        &self.front
    }
}

impl<Px: Default + Clone> SimpleRenderer<Px> {
    /// Create a new `SimpleRenderer` with the specified size. Buffers are
    /// allocated on the heap.
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
