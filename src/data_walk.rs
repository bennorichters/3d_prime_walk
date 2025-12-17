use crate::{
    color_gradient::ColorGradient,
    space::{Pixel3D, Tuple3D},
};
use std::fs;

pub fn walk(_steps: usize, _gradient: ColorGradient, start_color: (u8, u8, u8), end_color: (u8, u8, u8)) -> Vec<Pixel3D> {
    // Read the data file
    let contents = fs::read_to_string("data")
        .expect("Failed to read data file");

    // First pass: collect all valid coordinates
    let mut coordinates = vec![];

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            eprintln!("Invalid line format: {}", line);
            continue;
        }

        // Parse X, Y, Z coordinates
        let x = parts[0].trim().parse::<f64>()
            .expect(&format!("Failed to parse X coordinate: {}", parts[0]));
        let y = parts[1].trim().parse::<f64>()
            .expect(&format!("Failed to parse Y coordinate: {}", parts[1]));
        let z = parts[2].trim().parse::<f64>()
            .expect(&format!("Failed to parse Z coordinate: {}", parts[2]));

        coordinates.push(Tuple3D { x, y, z });
    }

    // Create gradient with the correct number of steps based on actual data points
    let data_point_count = coordinates.len();
    let mut gradient = ColorGradient::new(start_color, end_color, data_point_count);

    // Second pass: create pixels with gradient colors
    let mut result = vec![];
    for coordinate in coordinates {
        let color = gradient.next().unwrap();
        result.push(Pixel3D {
            coordinate,
            color,
        });
    }

    result
}
