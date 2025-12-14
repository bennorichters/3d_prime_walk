use crate::{color_gradient::ColorGradient, prime_walk::walk};

mod app;
mod camera;
mod color_gradient;
mod prime_walk;
mod primes;
mod space;

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
