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

    let pixels = walk(steps, gradient);
    show_extremes(&pixels);

    let file = File::create("output.gif").unwrap();
    let mut encoder = Encoder::new(file, size as u16, size as u16, &[]).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    let projection_it = PrimeMeridian::new();

    for (c, projection) in projection_it.enumerate() {
        println!("{}", c);

        let pixels2d = map_to_pixels2d(&pixels, projection);
        let imgbuf = image(pixels2d);

        let rgba = DynamicImage::ImageRgb8(imgbuf).to_rgba8();
        let mut frame = Frame::from_rgba_speed(size as u16, size as u16, &mut rgba.into_raw(), 10);
        frame.delay = delay;
        encoder.write_frame(&frame).unwrap();
    }

    drop(encoder);
}

#[derive(Debug)]
struct Pixel2D {
    x: i16,
    y: i16,
    color: (u8, u8, u8),
    distance: f64,
}

struct Pixel3D {
    coordinate: Tuple3D,
    color: (u8, u8, u8),
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

fn show_extremes(pixels: &[Pixel3D]) {
    let (min_x, max_x) = extremes(pixels, |e| e.coordinate.x);
    let (min_y, max_y) = extremes(pixels, |e| e.coordinate.y);
    let (min_z, max_z) = extremes(pixels, |e| e.coordinate.z);

    println!(
        "({}, {}, {}), ({}, {}, {})",
        min_x, min_y, min_z, max_x, max_y, max_z
    );
}

fn extremes<F>(pixels: &[Pixel3D], f: F) -> (f64, f64)
where
    F: Fn(&&Pixel3D) -> f64,
{
    let compare = |a: &&Pixel3D, b: &&Pixel3D| f(a).total_cmp(&f(b));
    let min = pixels.iter().min_by(compare).unwrap();
    let max = pixels.iter().max_by(compare).unwrap();

    (f(&min), f(&max))
}

fn map_to_pixels2d(pixels: &[Pixel3D], projection: Projection) -> Vec<Pixel2D> {
    let mut pixels2d: Vec<Pixel2D> = vec![];
    for pixel in pixels {
        let dist_coord_option = projection.project(&pixel.coordinate);
        if let Some((distance, coord)) = dist_coord_option {
            let x = coord.0.round() as i16;
            let y = coord.1.round() as i16;

            let index_option = pixels2d.iter().position(|e| e.x == x && e.y == y);
            let mut to_push = true;
            if let Some(index) = index_option {
                let existing = &pixels2d[index];
                if distance < existing.distance {
                    pixels2d.remove(index);
                } else {
                    to_push = false;
                }
            }

            let color = pixel.color;
            if to_push {
                pixels2d.push(Pixel2D {
                    x,
                    y,
                    color,
                    distance,
                });
            }
        }
    }
    pixels2d
}

fn image(pixels2d: Vec<Pixel2D>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let size = 500;
    let half_size = (size / 2) as i16;
    let mut imgbuf = ImageBuffer::new(size, size);

    for d in pixels2d {
        if d.x >= -half_size && d.x < half_size && d.y >= -half_size && d.y < half_size {
            let x = (half_size + d.x) as u32;
            let y = (half_size + d.y) as u32;

            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = Rgb([d.color.0, d.color.1, d.color.2]);
        }
    }

    imgbuf
}

fn walk(steps: usize, mut gradient: ColorGradient) -> Vec<Pixel3D> {
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
        result.push(Pixel3D {
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
