#![cfg_attr(all(test, feature = "nightly"), feature(test))]

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

pub use renderer::{Coord, Drawable, Renderer};
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
    use test::Bencher;

    use super::{Rectangle, Triangle};

    #[test]
    fn rect() {
        assert_eq!(
            Rectangle::new(0, 2, 0, 2).into_iter().collect::<Vec<_>>(),
            [(0, 0), (1, 0), (0, 1), (1, 1),]
        )
    }

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
}
