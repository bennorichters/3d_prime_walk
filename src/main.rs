mod app;
mod camera;
mod color_gradient;
mod primes;
mod space;

use crate::{color_gradient::ColorGradient, primes::Primes, space::Tuple3D};

pub const SIZE: usize = 800;
const DEFAULT_STEPS: usize = 25_000;
const DEFAULT_START_COLOR: (u8, u8, u8) = (255, 0, 0); // Red
const DEFAULT_END_COLOR: (u8, u8, u8) = (0, 0, 255); // Blue

fn parse_color(s: &str) -> Option<(u8, u8, u8)> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return None;
    }

    let r = parts[0].parse::<u8>().ok()?;
    let g = parts[1].parse::<u8>().ok()?;
    let b = parts[2].parse::<u8>().ok()?;

    Some((r, g, b))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let steps = args
        .get(1)
        .and_then(|arg| arg.parse::<usize>().ok())
        .unwrap_or(DEFAULT_STEPS);

    let start_color = args
        .get(2)
        .and_then(|arg| parse_color(arg))
        .unwrap_or(DEFAULT_START_COLOR);

    let end_color = args
        .get(3)
        .and_then(|arg| parse_color(arg))
        .unwrap_or(DEFAULT_END_COLOR);

    let gradient = ColorGradient::new(start_color, end_color, steps);
    let pixels = walk(steps, gradient);

    app::image(pixels);
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
