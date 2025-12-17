use crate::color_gradient::ColorGradient;
use clap::Parser;

mod app;
mod camera;
mod color_gradient;
mod cube;
mod data_walk;
mod prime_walk;
mod primes;
mod space;

pub const SIZE: usize = 800;
const DEFAULT_STEPS: usize = 25_000;
const DEFAULT_CAMERA_RADIUS: f64 = 600.0;
const DEFAULT_FOCAL_LENGTH: f64 = 600.0;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'n', long, default_value_t = DEFAULT_STEPS)]
    steps: usize,

    #[arg(short = 's', long, value_parser = parse_color, default_value = "255,0,0")]
    start_color: (u8, u8, u8),

    #[arg(short = 'e', long, value_parser = parse_color, default_value = "0,0,255")]
    end_color: (u8, u8, u8),

    #[arg(short = 'w', long, default_value = "prime_walk")]
    walk_type: String,
}

fn parse_color(s: &str) -> Result<(u8, u8, u8), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return Err(format!("Color must be in R,G,B format, got: {}", s));
    }

    let r = parts[0]
        .parse::<u8>()
        .map_err(|_| format!("Invalid red value: {}", parts[0]))?;
    let g = parts[1]
        .parse::<u8>()
        .map_err(|_| format!("Invalid green value: {}", parts[1]))?;
    let b = parts[2]
        .parse::<u8>()
        .map_err(|_| format!("Invalid blue value: {}", parts[2]))?;

    Ok((r, g, b))
}

fn main() {
    let args = Args::parse();

    if args.walk_type == "data_walk" && args.steps != DEFAULT_STEPS {
        eprintln!("Warning: --steps/-n argument is ignored in data_walk mode. The number of points is determined by the data file.");
    }

    let gradient = ColorGradient::new(args.start_color, args.end_color, args.steps);

    let pixels = match args.walk_type.as_str() {
        "cube" => cube::walk(args.steps, gradient),
        "data_walk" => data_walk::walk(args.steps, gradient, args.start_color, args.end_color),
        _ => prime_walk::walk(args.steps, gradient),
    };

    app::image(pixels, DEFAULT_CAMERA_RADIUS, DEFAULT_FOCAL_LENGTH);
}
