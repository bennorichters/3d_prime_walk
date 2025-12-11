use crate::space::{Plane, Tuple3D};
use std::f64::consts::PI;

const FULL_CIRCLE: u16 = 360;
const HALF_CIRCLE: u16 = 180;

fn rad(angle: u16) -> f64 {
    (angle as f64 * PI) / HALF_CIRCLE as f64
}

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

pub struct Orbit {
    polar: u16,
    azimnuth: u16,
    camera_radius: f64,
    focal_length: f64,
}

impl Orbit {
    pub fn new(camera_radius: f64, focal_length: f64) -> Self {
        Orbit {
            polar: 0,
            azimnuth: 0,
            camera_radius,
            focal_length,
        }
    }

    pub fn projection(&self) -> Projection {
        let a = rad(self.azimnuth);
        let p = rad(self.polar);

        let vec_x = a.sin() * p.cos();
        let vec_y = p.sin();
        let vec_z = a.cos() * p.cos();

        let camera = Tuple3D {
            x: self.camera_radius * vec_x,
            y: self.camera_radius * vec_y,
            z: self.camera_radius * vec_z,
        };

        let screen_radius = self.camera_radius - self.focal_length;
        let screen_coordinate = Tuple3D {
            x: screen_radius * vec_x,
            y: screen_radius * vec_y,
            z: screen_radius * vec_z,
        };

        let vector_u = Tuple3D {
            x: a.cos(),
            y: 0.0,
            z: -a.sin(),
        };

        let vector_v = Tuple3D {
            x: -a.sin() * p.sin(),
            y: p.cos(),
            z: -a.cos() * p.sin(),
        };

        Projection {
            camera,
            screen: Plane {
                coordinate: screen_coordinate,
                vector_u,
                vector_v,
            },
        }
    }

    pub fn inc_polar(&mut self) -> Projection {
        if self.polar == FULL_CIRCLE - 1 {
            self.polar = 0;
        } else {
            self.polar += 1;
        }

        self.projection()
    }

    pub fn dec_polar(&mut self) -> Projection {
        if self.polar == 0 {
            self.polar = FULL_CIRCLE - 1;
        } else {
            self.polar -= 1;
        }

        self.projection()
    }

    pub fn inc_azimuth(&mut self) -> Projection {
        if self.azimnuth == FULL_CIRCLE - 1 {
            self.azimnuth = 0;
        } else {
            self.azimnuth += 1;
        }

        self.projection()
    }

    pub fn dec_azimuth(&mut self) -> Projection {
        if self.azimnuth == 0 {
            self.azimnuth = FULL_CIRCLE - 1;
        } else {
            self.azimnuth -= 1;
        }

        self.projection()
    }

    pub fn polar(&self) -> u16 {
        self.polar
    }

    pub fn azimuth(&self) -> u16 {
        self.azimnuth
    }

    pub fn inc_camera_radius(&mut self) -> Projection {
        self.camera_radius += 1.0;
        self.projection()
    }

    pub fn dec_camera_radius(&mut self) -> Projection {
        self.camera_radius -= 1.0;
        self.projection()
    }

    pub fn inc_focal_length(&mut self) -> Projection {
        self.focal_length += 1.0;
        self.projection()
    }

    pub fn dec_focal_length(&mut self) -> Projection {
        self.focal_length -= 1.0;
        self.projection()
    }
}
