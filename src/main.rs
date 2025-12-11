#![allow(dead_code)]

mod app;
mod camera;
mod color_gradient;
mod primes;
mod space;

use eframe::egui;

use crate::{camera::*, color_gradient::ColorGradient, primes::Primes, space::Tuple3D};

pub const SIZE: usize = 800;
const HALF_SIZE: isize = SIZE as isize / 2;

fn main() {
    let steps = 10_000;
    let gradient = ColorGradient::new((255, 0, 0), (0, 0, 255), steps);

    let pixels = walk(steps, gradient);
    show_extremes(&pixels);

    app::image(pixels);
}

#[derive(Debug)]
struct Pixel2D {
    x: i16,
    y: i16,
    color: (u8, u8, u8),
    distance: f64,
}

pub struct Pixel3D {
    pub coordinate: Tuple3D,
    pub color: (u8, u8, u8),
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

pub fn map_to_pixels2d(pixels3d: &[Pixel3D], projection: Projection) -> egui::ColorImage {
    let mut pixels2d: Vec<egui::Color32> = vec![egui::Color32::BLACK; SIZE * SIZE];
    let mut distances: Vec<f64> = vec![f64::MAX; SIZE * SIZE];

    for pixel3d in pixels3d {
        let dist_coord_option = projection.project(&pixel3d.coordinate);
        if let Some((distance, coord)) = dist_coord_option {
            let ix = HALF_SIZE + coord.0.round() as isize;
            let iy = HALF_SIZE + coord.1.round() as isize;

            if ix >= 0 && iy >= 0 {
                let x = ix as usize;
                let y = iy as usize;

                if x < SIZE && y < SIZE {
                    let index = y * SIZE + x;
                    if distance < distances[index] {
                        pixels2d[index] = egui::Color32::from_rgb(
                            pixel3d.color.0,
                            pixel3d.color.1,
                            pixel3d.color.2,
                        );
                        distances[index] = distance;
                    }
                }
            }
        }
    }

    egui::ColorImage {
        size: [SIZE, SIZE],
        source_size: egui::Vec2::new(SIZE as f32, SIZE as f32),
        pixels: pixels2d,
    }
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
