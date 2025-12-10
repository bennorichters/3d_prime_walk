#![allow(dead_code)]

mod camera;
mod color_gradient;
mod primes;
mod space;

use std::fs::File;

use image::{DynamicImage, ImageBuffer, Rgb};

use gif::{Encoder, Frame, Repeat};

use crate::{
    camera::{PrimeMeridian, Projection},
    color_gradient::ColorGradient,
    primes::Primes,
    space::Tuple3D,
};

fn main() {
    let steps = 10_000;
    let gradient = ColorGradient::new((255, 0, 0), (0, 0, 255), steps);
    let size = 500;
    let delay = 4;

    let dots = walk(steps, gradient);
    show_extremes(&dots);

    let file = File::create("output.gif").unwrap();
    let mut encoder = Encoder::new(file, size as u16, size as u16, &[]).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    let projection_it = PrimeMeridian::new();

    for (c, projection) in projection_it.enumerate() {
        println!("{}", c);

        let dot2ds = map_to_dot2d(&dots, projection);
        let imgbuf = image(dot2ds);

        let rgba = DynamicImage::ImageRgb8(imgbuf).to_rgba8();
        let mut frame = Frame::from_rgba_speed(size as u16, size as u16, &mut rgba.into_raw(), 10);
        frame.delay = delay;
        encoder.write_frame(&frame).unwrap();
    }

    drop(encoder);
}

fn show_extremes(dots: &[Dot3D]) {
    let (min_x, max_x) = extremes(dots, |e| e.coordinate.x);
    let (min_y, max_y) = extremes(dots, |e| e.coordinate.y);
    let (min_z, max_z) = extremes(dots, |e| e.coordinate.z);

    println!(
        "({}, {}, {}), ({}, {}, {})",
        min_x, min_y, min_z, max_x, max_y, max_z
    );
}

fn extremes<F>(dots: &[Dot3D], f: F) -> (f64, f64)
where
    F: Fn(&&Dot3D) -> f64,
{
    let compare = |a: &&Dot3D, b: &&Dot3D| f(a).total_cmp(&f(b));
    let min = dots.iter().min_by(compare).unwrap();
    let max = dots.iter().max_by(compare).unwrap();

    (f(&min), f(&max))
}

fn map_to_dot2d(dots: &[Dot3D], projection: Projection) -> Vec<Dot2D> {
    let mut dot2ds: Vec<Dot2D> = vec![];
    for dot in dots {
        let dist_coord_option = projection.project(&dot.coordinate);
        if let Some((distance, coord)) = dist_coord_option {
            let x = coord.0.round() as i16;
            let y = coord.1.round() as i16;

            let index_option = dot2ds.iter().position(|e| e.x == x && e.y == y);
            let mut to_push = true;
            if let Some(index) = index_option {
                let existing = &dot2ds[index];
                if distance < existing.distance {
                    dot2ds.remove(index);
                } else {
                    to_push = false;
                }
            }

            let color = dot.color;
            if to_push {
                dot2ds.push(Dot2D {
                    x,
                    y,
                    color,
                    distance,
                });
            }
        }
    }
    dot2ds
}

fn image(dot2ds: Vec<Dot2D>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let size = 500;
    let half_size = (size / 2) as i16;
    let mut imgbuf = ImageBuffer::new(size, size);

    for d in dot2ds {
        if d.x >= -half_size && d.x < half_size && d.y >= -half_size && d.y < half_size {
            let x = (half_size + d.x) as u32;
            let y = (half_size + d.y) as u32;

            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = Rgb([d.color.0, d.color.1, d.color.2]);
        }
    }

    imgbuf
}

#[derive(Debug)]
struct Dot2D {
    x: i16,
    y: i16,
    color: (u8, u8, u8),
    distance: f64,
}

static DIRS: &[[isize; 3]] = &[
    [1, 0, 0],
    [0, 1, 0],
    [0, 0, 1],
    [-1, 0, 0],
    [0, -1, 0],
    [0, 0, -1],
];

struct DirIterator {
    index: usize,
}

impl Iterator for DirIterator {
    type Item = [isize; 3];

    fn next(&mut self) -> Option<Self::Item> {
        let result = DIRS[self.index];
        self.index += 1;
        if self.index == DIRS.len() {
            self.index = 0;
        }

        Some(result)
    }
}

fn walk(steps: usize, mut gradient: ColorGradient) -> Vec<Dot3D> {
    let mut result = vec![];

    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    let mut dir_it = DirIterator { index: 0 };

    let mut dir = dir_it.next().unwrap();
    let mut primes = Primes::new();
    let mut p = primes.next().unwrap();
    for n in 0..steps {
        if n == (p as usize) {
            dir = dir_it.next().unwrap();
            p = primes.next().unwrap();
        }

        let color = gradient.next().unwrap();
        result.push(Dot3D {
            coordinate: Tuple3D {
                x: x as f64,
                y: y as f64,
                z: z as f64,
            },
            color,
        });

        x += dir[0];
        y += dir[1];
        z += dir[2];
    }

    result
}

struct Dot3D {
    coordinate: Tuple3D,
    color: (u8, u8, u8),
}
