use std::fs::File;

use image::{codecs::gif::GifEncoder, Delay, Frame, ImageBuffer};

fn main() {
    let center = Point3D {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let radius = 150.0;
    let elevation = 0.0;
    let focal_length = 100.0;

    let steps = 10_000;

    let dots = walk(steps);
    show_extremes(&dots);

    let mut frames = Vec::new();
    for a in 0..360 {
        println!("{}", a);
        let azimuth = a as f64 * std::f64::consts::PI / 180.0;
        let gradient = ColorGradient::new((255, 0, 0), (0, 0, 255), steps);
        let dot2ds = map_to_dot2d(
            &dots,
            center,
            radius,
            azimuth,
            elevation,
            focal_length,
            gradient,
        );
        let imgbuf = image(dot2ds);
        let rgba: image::RgbaImage = image::DynamicImage::ImageRgb8(imgbuf).to_rgba8();
        frames.push(Frame::from_parts(
            rgba,
            0,
            0,
            Delay::from_numer_denom_ms(100, 1),
        ));
    }

    let file = File::create("output.gif").unwrap();
    let mut encoder = GifEncoder::new(file);

    let _ = encoder.encode_frames(frames);
    drop(encoder);
}

fn show_extremes(dots: &[Point3D]) {
    let (min_x, max_x) = extremes(dots, |e| e.x);
    let (min_y, max_y) = extremes(dots, |e| e.y);
    let (min_z, max_z) = extremes(dots, |e| e.z);

    println!(
        "({}, {}, {}), ({}, {}, {})",
        min_x, min_y, min_z, max_x, max_y, max_z
    );
}

fn extremes<F>(dots: &[Point3D], f: F) -> (f64, f64)
where
    F: Fn(&&Point3D) -> f64,
{
    let compare = |a: &&Point3D, b: &&Point3D| f(a).total_cmp(&f(b));
    let min = dots.iter().min_by(compare).unwrap();
    let max = dots.iter().max_by(compare).unwrap();

    (f(&min), f(&max))
}

fn map_to_dot2d(
    dots: &[Point3D],
    center: Point3D,
    radius: f64,
    azimmuth: f64,
    elevation: f64,
    focal_length: f64,
    mut gradient: ColorGradient,
) -> Vec<Dot2D> {
    let mut dot2ds: Vec<Dot2D> = vec![];
    for dot in dots {
        let color = gradient.next().unwrap();
        let (viewpoint, coord_option) =
            orbit_project(dot, center, radius, azimmuth, elevation, focal_length);
        if let Some(coord) = coord_option {
            let x = coord[0].round() as i16;
            let y = coord[1].round() as i16;
            let distance = viewpoint.distance_to(dot);

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

fn image(dot2ds: Vec<Dot2D>) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let size = 500;
    let half_size = (size / 2) as i16;
    let mut imgbuf = image::ImageBuffer::new(size, size);

    for d in dot2ds {
        if d.x >= -half_size && d.x < half_size && d.y >= -half_size && d.y < half_size {
            let x = (half_size + d.x) as u32;
            let y = (half_size + d.y) as u32;

            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb([d.color.0, d.color.1, d.color.2]);
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

struct ColorGradient {
    current_step: usize,
    total_steps: usize,
    start: (f64, f64, f64),
    end: (f64, f64, f64),
}

impl ColorGradient {
    fn new(start: (u8, u8, u8), end: (u8, u8, u8), steps: usize) -> Self {
        Self {
            current_step: 0,
            total_steps: steps,
            start: (start.0 as f64, start.1 as f64, start.2 as f64),
            end: (end.0 as f64, end.1 as f64, end.2 as f64),
        }
    }
}

impl Iterator for ColorGradient {
    type Item = (u8, u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_step >= self.total_steps {
            return None;
        }

        let t = if self.total_steps == 1 {
            0.0
        } else {
            self.current_step as f64 / (self.total_steps - 1) as f64
        };

        let r = (self.start.0 + (self.end.0 - self.start.0) * t).round() as u8;
        let g = (self.start.1 + (self.end.1 - self.start.1) * t).round() as u8;
        let b = (self.start.2 + (self.end.2 - self.start.2) * t).round() as u8;

        self.current_step += 1;
        Some((r, g, b))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.total_steps - self.current_step;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for ColorGradient {}

fn walk(steps: usize) -> Vec<Point3D> {
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

        result.push(Point3D {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        });

        x += dir[0];
        y += dir[1];
        z += dir[2];
    }

    result
}

struct Primes {
    primes: Vec<u64>,
}

impl Primes {
    fn new() -> Self {
        Primes { primes: Vec::new() }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let candidate = match self.primes.last() {
            None => 2,
            Some(&2) => 3,
            Some(&p) => {
                let mut c = p + 2;
                while !self.is_prime(c) {
                    c += 2;
                }
                c
            }
        };
        self.primes.push(candidate);
        Some(candidate)
    }
}

impl Primes {
    fn is_prime(&self, n: u64) -> bool {
        for &p in &self.primes {
            if p * p > n {
                return true;
            }
            if n.is_multiple_of(p) {
                return false;
            }
        }
        true
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    fn distance_to(&self, other: &Point3D) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn project_with_rotation(
    point: &Point3D,
    camera_pos: Point3D,
    rotation: [[f64; 3]; 3],
    focal_length: f64,
) -> Option<[f64; 2]> {
    // Translate
    let t = [
        point.x - camera_pos.x,
        point.y - camera_pos.y,
        point.z - camera_pos.z,
    ];

    // Rotate
    let x = rotation[0][0] * t[0] + rotation[0][1] * t[1] + rotation[0][2] * t[2];
    let y = rotation[1][0] * t[0] + rotation[1][1] * t[1] + rotation[1][2] * t[2];
    let z = rotation[2][0] * t[0] + rotation[2][1] * t[1] + rotation[2][2] * t[2];

    if z <= 0.0 {
        return None;
    }

    Some([focal_length * x / z, focal_length * y / z])
}

fn orbit_project(
    point: &Point3D,
    target: Point3D,
    radius: f64,
    azimuth: f64,   // horizontal angle (radians)
    elevation: f64, // vertical angle (radians)
    focal_length: f64,
) -> (Point3D, Option<[f64; 2]>) {
    let cam_x = target.x + radius * elevation.cos() * azimuth.cos();
    let cam_y = target.y + radius * elevation.sin();
    let cam_z = target.z + radius * elevation.cos() * azimuth.sin();
    let camera_pos = Point3D {
        x: cam_x,
        y: cam_y,
        z: cam_z,
    };

    // Calculate look-at rotation matrix
    // Forward: camera -> target (normalized)
    let forward = [target.x - cam_x, target.y - cam_y, target.z - cam_z];
    let f_len = (forward[0].powi(2) + forward[1].powi(2) + forward[2].powi(2)).sqrt();
    let f = [forward[0] / f_len, forward[1] / f_len, forward[2] / f_len];

    // Up reference (world up)
    let up = [0.0, 1.0, 0.0];

    // Right: up × forward
    let r = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0],
    ];
    let r_len = (r[0].powi(2) + r[1].powi(2) + r[2].powi(2)).sqrt();
    let r = [r[0] / r_len, r[1] / r_len, r[2] / r_len];

    // Recalculate up: forward × right
    let u = [
        f[1] * r[2] - f[2] * r[1],
        f[2] * r[0] - f[0] * r[2],
        f[0] * r[1] - f[1] * r[0],
    ];

    // Rotation matrix (rows are right, up, forward)
    let rotation = [[r[0], r[1], r[2]], [u[0], u[1], u[2]], [f[0], f[1], f[2]]];

    (
        camera_pos,
        project_with_rotation(point, camera_pos, rotation, focal_length),
    )
}
