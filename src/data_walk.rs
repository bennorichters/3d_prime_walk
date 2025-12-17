use crate::{
    color_gradient::ColorGradient,
    space::{Pixel3D, Tuple3D},
};
use std::fs;

pub fn walk(_steps: usize, mut gradient: ColorGradient) -> Vec<Pixel3D> {
    let mut result = vec![];

    // Read the data file
    let contents = fs::read_to_string("data")
        .expect("Failed to read data file");

    // Parse each line as X,Y,Z coordinates
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

        // Get the next color from the gradient
        let color = gradient.next().unwrap();

        // Create Pixel3D with gradient color
        result.push(Pixel3D {
            coordinate: Tuple3D { x, y, z },
            color,
        });
    }

    result
}
