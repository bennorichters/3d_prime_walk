use crate::{
    color_gradient::ColorGradient,
    space::{Pixel3D, Tuple3D},
};

pub fn walk(_steps: usize, mut _gradient: ColorGradient) -> Vec<Pixel3D> {
    let mut result = vec![];

    square(&mut result, 0.0, (255, 0, 0));
    for z in (-10..=0).rev() {
        result.push(Pixel3D {
            coordinate: Tuple3D {
                x: -5.0,
                y: 5.0,
                z: z as f64,
            },
            color: (0, 255, 0),
        });
    }
    square(&mut result, -10.0, (0, 0, 255));

    result
}

fn square(result: &mut Vec<Pixel3D>, z: f64, color: (u8, u8, u8)) {
    for x in -5..=5 {
        result.push(Pixel3D {
            coordinate: Tuple3D {
                x: x as f64,
                y: 5.0,
                z,
            },
            color,
        });
    }

    for y in (-5..=5).rev() {
        result.push(Pixel3D {
            coordinate: Tuple3D {
                x: 5.0,
                y: y as f64,
                z,
            },
            color,
        });
    }

    for x in (-5..=5).rev() {
        result.push(Pixel3D {
            coordinate: Tuple3D {
                x: x as f64,
                y: -5.0,
                z,
            },
            color,
        });
    }

    for y in -5..=5 {
        result.push(Pixel3D {
            coordinate: Tuple3D {
                x: -5.0,
                y: y as f64,
                z,
            },
            color,
        });
    }
}
