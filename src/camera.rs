use crate::space::{Plane, Tuple3D};
use std::f64::consts::PI;

const FULL_CIRCLE: u16 = 360;
const HALF_CIRCLE: u16 = 180;
const CAMERA_RADIUS: f64 = 150.0;
const SCREEN_RADIUS: f64 = 100.0;

pub struct Projection {
    pub camera: Tuple3D,
    pub screen: Plane,
}

impl Projection {
    pub fn project(&self, target: &Tuple3D) -> Option<(f64, (f64, f64))> {
        let relative_option = self.screen.intersect(&self.camera, target);
        if let Some(relative_coords) = relative_option {
            let distance = self.camera.coordinate_squared_distance(target);

            return Some((distance, relative_coords));
        }

        None
    }
}

pub struct Equator {
    angle: u16,
}

impl Equator {
    pub fn new() -> Self {
        Equator { angle: 0 }
    }
}

impl Iterator for Equator {
    type Item = Projection;

    fn next(&mut self) -> Option<Self::Item> {
        if self.angle == FULL_CIRCLE {
            return None;
        }

        let rad: f64 = (self.angle as f64 * PI) / HALF_CIRCLE as f64;
        let sin = rad.sin();
        let cos = rad.cos();

        let camera = Tuple3D {
            x: sin * CAMERA_RADIUS,
            y: 0.0,
            z: cos * CAMERA_RADIUS,
        };

        let screen = Plane {
            coordinate: Tuple3D {
                x: sin * SCREEN_RADIUS,
                y: 0.0,
                z: cos * SCREEN_RADIUS,
            },
            vector1: Tuple3D {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            vector2: Tuple3D {
                x: cos,
                y: 0.0,
                z: -sin,
            },
        };

        self.angle += 1;

        Some(Projection { camera, screen })
    }
}

pub struct PrimeMeridian {
    angle: u16,
}

impl PrimeMeridian {
    pub fn new() -> Self {
        PrimeMeridian { angle: 0 }
    }
}

impl Iterator for PrimeMeridian {
    type Item = Projection;

    fn next(&mut self) -> Option<Self::Item> {
        if self.angle == FULL_CIRCLE {
            return None;
        }

        let rad: f64 = (self.angle as f64 * PI) / HALF_CIRCLE as f64;
        let sin = rad.sin();
        let cos = rad.cos();

        let camera = Tuple3D {
            x: 0.0,
            y: sin * CAMERA_RADIUS,
            z: cos * CAMERA_RADIUS,
        };

        let screen = Plane {
            coordinate: Tuple3D {
                x: 0.0,
                y: sin * SCREEN_RADIUS,
                z: cos * SCREEN_RADIUS,
            },
            vector1: Tuple3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            vector2: Tuple3D {
                x: 0.0,
                y: cos,
                z: -sin,
            },
        };

        self.angle += 1;

        Some(Projection { camera, screen })
    }
}

pub struct EastWest {
    x: i16,
}

impl EastWest {
    pub fn new() -> Self {
        EastWest { x: -50 }
    }
}

impl Iterator for EastWest {
    type Item = Projection;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == 350 {
            return None;
        }

        let camera = Tuple3D {
            x: self.x as f64,
            y: 0.0,
            z: -100.0,
        };

        let screen = Plane {
            coordinate: Tuple3D {
                x: self.x as f64,
                y: 0.0,
                z: -50.0,
            },
            vector1: Tuple3D {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            vector2: Tuple3D {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };

        self.x += 1;
        Some(Projection { camera, screen })
    }
}
