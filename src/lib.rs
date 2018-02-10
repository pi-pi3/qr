#![allow(unknown_lints)]
#![allow(inline_always)]
#![cfg_attr(feature = "check-docs", deny(missing_docs))]
#![cfg_attr(all(test, feature = "nightly"), feature(test))]

//! # qr - A 2D/3D software rendering library
//!
//! # Example
//!
//! ```rust,norun
//! extern crate qr;
//!
//! use std::iter;
//!
//! use qr::{Triangle, Renderer, SimpleRenderer};
//!
//! const WIDTH: usize = 128;
//! const HEIGHT: usize = 128;
//!
//! fn main() {
//!     let mut renderer = SimpleRenderer::new(WIDTH, HEIGHT);
//!     let triangle = Triangle::with_points([(0.0, 0.0), (100.0, 100.0), (0.0, 100.0)]);
//!     let mesh = iter::once(triangle);
//!
//!     renderer.set_attr(0, (255_u8, 255_u8, 255_u8));
//!     if let Ok((s, v, f)) = renderer.draw(mesh) {
//!         println!("drawn {} primitives, {} vertices and {} fragments", s, v, f);
//!     }
//! }
//! ```

extern crate image;
extern crate line_drawing;
extern crate num_traits;
extern crate rand;
#[cfg(all(test, feature = "nightly"))]
extern crate test;

pub mod renderer;
pub mod shape;
pub mod point;
pub mod line;
pub mod rect;
pub mod tri;

pub use renderer::{Coord, Drawable, Renderer, SimpleRenderer};
pub use shape::Shape;
pub use point::{Point, Point2};
pub use line::Line;
pub use rect::Rectangle;
pub use tri::Triangle;

#[cfg(test)]
mod tests {
    use rand;
    use image::{ImageBuffer, Pixel, Rgb};

    #[cfg(feature = "nightly")]
    use test::{black_box, Bencher};

    use super::Triangle;

    #[cfg(feature = "nightly")]
    #[bench]
    fn rgb_tri(b: &mut Bencher) {
        const BPP: usize = 24;
        const WIDTH: usize = 1024;
        const HEIGHT: usize = 1024;
        const SIZE: usize = WIDTH * HEIGHT * BPP / 8;

        let buffer = vec![0_u8; SIZE];
        let image = ImageBuffer::from_raw(WIDTH as _, HEIGHT as _, buffer);
        let mut image = image.unwrap();

        let width = WIDTH as f64 - 1.0;
        let height = HEIGHT as f64 - 1.0;

        b.iter(|| {
            let x0 = rand::random::<f64>() * width as f64;
            let x1 = rand::random::<f64>() * width as f64;
            let x2 = rand::random::<f64>() * width as f64;

            let y0 = rand::random::<f64>() * height as f64;
            let y1 = rand::random::<f64>() * height as f64;
            let y2 = rand::random::<f64>() * height as f64;

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
        });

        image.save("bench1.png").expect("couldn't save image");
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn white_tri(b: &mut Bencher) {
        const BPP: usize = 24;
        const WIDTH: usize = 1024;
        const HEIGHT: usize = 1024;
        const SIZE: usize = WIDTH * HEIGHT * BPP / 8;

        let buffer = vec![0_u8; SIZE];
        let image = ImageBuffer::from_raw(WIDTH as _, HEIGHT as _, buffer);
        let mut image = image.unwrap();

        let width = WIDTH as f64 - 1.0;
        let height = HEIGHT as f64 - 1.0;

        b.iter(|| {
            let x0 = rand::random::<f64>() * width as f64;
            let x1 = rand::random::<f64>() * width as f64;
            let x2 = rand::random::<f64>() * width as f64;

            let y0 = rand::random::<f64>() * height as f64;
            let y1 = rand::random::<f64>() * height as f64;
            let y2 = rand::random::<f64>() * height as f64;

            let v1 = (x0, y0);
            let v2 = (x1, y1);
            let v3 = (x2, y2);

            let triangle = Triangle::with_points([v1, v2, v3]);

            triangle
                .into_iter()
                .map(|((x, y), b)| ((x as u32, y as u32), b))
                .for_each(|((x, y), _)| {
                    image.put_pixel(x, y, Rgb::from_channels(255, 255, 255, 255));
                });
        });

        image.save("bench2.png").expect("couldn't save image");
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn small_tris(b: &mut Bencher) {
        const BPP: usize = 24;
        const WIDTH: usize = 1024;
        const HEIGHT: usize = 1024;
        const SIZE: usize = WIDTH * HEIGHT * BPP / 8;

        let buffer = vec![0_u8; SIZE];
        let image = ImageBuffer::from_raw(WIDTH as _, HEIGHT as _, buffer);
        let mut image = image.unwrap();

        let size = 16.0;

        b.iter(|| {
            let x = rand::random::<f64>() * (WIDTH as f64 - size);
            let y = rand::random::<f64>() * (HEIGHT as f64 - size);

            let size = size - 1.0;

            let x0 = rand::random::<f64>() * size + x;
            let x1 = rand::random::<f64>() * size + x;
            let x2 = rand::random::<f64>() * size + x;

            let y0 = rand::random::<f64>() * size + y;
            let y1 = rand::random::<f64>() * size + y;
            let y2 = rand::random::<f64>() * size + y;

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

            (0..1000).for_each(|_| {
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
            });
        });

        image.save("bench3.png").expect("couldn't save image");
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn attr8_tri(b: &mut Bencher) {
        let dot = |(ax, ay, az), (bx, by, bz)| ax * bx + ay * by + az * bz;
        let rand = |m| {
            (
                rand::random::<f64>() * m,
                rand::random::<f64>() * m,
                rand::random::<f64>() * m,
            )
        };

        const BPP: usize = 24;
        const WIDTH: usize = 1024;
        const HEIGHT: usize = 1024;
        const SIZE: usize = WIDTH * HEIGHT * BPP / 8;

        let buffer = vec![0_u8; SIZE];
        let image = ImageBuffer::from_raw(WIDTH as _, HEIGHT as _, buffer);
        let mut image = image.unwrap();

        let width = WIDTH as f64 - 1.0;
        let height = HEIGHT as f64 - 1.0;

        // light normal
        let (lx, ly, lz) = rand(1.0);
        let len_recip = (lx * lx + ly * ly + lz * lz).sqrt().recip();
        let (lx, ly, lz) = (lx * len_recip, ly * len_recip, lz * len_recip);

        b.iter(|| {
            // position
            let (x0, x1, x2) = rand(width as f64);
            let (y0, y1, y2) = rand(height as f64);

            // color
            let (r0, r1, r2) = rand(255.0);
            let (g0, g1, g2) = rand(255.0);
            let (b0, b1, b2) = rand(255.0);

            // normal
            let (nx0, nx1, nx2) = rand(1.0);
            let (ny0, ny1, ny2) = rand(1.0);
            let (nz0, nz1, nz2) = rand(1.0);

            // normalize normal
            let len_recip0 = (nx0 * nx0 + ny0 * ny0 + nz0 * nz0).sqrt().recip();
            let len_recip1 = (nx1 * nx1 + ny1 * ny1 + nz1 * nz1).sqrt().recip();
            let len_recip2 = (nx2 * nx2 + ny2 * ny2 + nz2 * nz2).sqrt().recip();

            let nx0 = nx0 * len_recip0;
            let nx1 = nx1 * len_recip1;
            let nx2 = nx2 * len_recip2;

            let ny0 = ny0 * len_recip0;
            let ny1 = ny1 * len_recip1;
            let ny2 = ny2 * len_recip2;

            let nz0 = nz0 * len_recip0;
            let nz1 = nz1 * len_recip1;
            let nz2 = nz2 * len_recip2;

            // texture coordinate
            let (tu0, tu1, tu2) = rand(1.0);
            let (tv0, tv1, tv2) = rand(1.0);

            let v1 = (x0, y0);
            let v2 = (x1, y1);
            let v3 = (x2, y2);

            let triangle = Triangle::with_points([v1, v2, v3]);

            triangle
                .into_iter()
                .map(|((x, y), b)| ((x as u32, y as u32), b))
                .for_each(|((x, y), p)| {
                    // diffuse color at (x, y)
                    let r = p[0] * r0 + p[1] * r1 + p[2] * r2;
                    let g = p[0] * g0 + p[1] * g1 + p[2] * g2;
                    let b = p[0] * b0 + p[1] * b1 + p[2] * b2;

                    // normal at (x, y)
                    let nx = p[0] * nx0 + p[1] * nx1 + p[2] * nx2;
                    let ny = p[0] * ny0 + p[1] * ny1 + p[2] * ny2;
                    let nz = p[0] * nz0 + p[1] * nz1 + p[2] * nz2;

                    // texture uv at (x, y)
                    let tu = p[0] * tu0 + p[1] * tu1 + p[2] * tu2;
                    let tv = p[0] * tv0 + p[1] * tv1 + p[2] * tv2;

                    // calculate reflectance according to the lambertian cosine law
                    let shade = -dot((-lx, -ly, -lz), (nx, ny, nz));
                    let r = r * shade;
                    let g = g * shade;
                    let b = b * shade;

                    // simulate sampling texture
                    let width = 1024;
                    let height = 1024;
                    let index =
                        (width as f64 * tu) as usize + (height as f64 * tv) as usize * width;
                    let _index = black_box(index);

                    image.put_pixel(x, y, Rgb::from_channels(r as u8, g as u8, b as u8, 255));
                });
        });

        image.save("bench4.png").expect("couldn't save image");
    }
}
