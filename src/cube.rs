use crate::{
    color_gradient::ColorGradient,
    space::{Pixel3D, Tuple3D},
};

pub fn walk(_steps: usize, mut _gradient: ColorGradient) -> Vec<Pixel3D> {
    let mut result = vec![];

    for x in -5..=5 {
        result.push(Pixel3D {
            coordinate: Tuple3D {
                x: x as f64,
                y: 10.0,
                z: 0.0,
            },
            color: (255, 0, 0),
        });
        result.push(Pixel3D {
            coordinate: Tuple3D {
                x: x as f64,
                y: -10.0,
                z: 0.0,
            },
            color: (255, 0, 0),
        });
    }

    for y in -5..=5 {
        result.push(Pixel3D {
            coordinate: Tuple3D {
                x: -10.0,
                y: y as f64,
                z: 0.0,
            },
            color: (255, 0, 0),
        });
        result.push(Pixel3D {
            coordinate: Tuple3D {
                x: 10.0,
                y: y as f64,
                z: 0.0,
            },
            color: (255, 0, 0),
        });
    }

    result
}
