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
    rotation: u16,
    camera_radius: f64,
    focal_length: f64,
    center: Tuple3D,
}

impl Orbit {
    pub fn new(camera_radius: f64, focal_length: f64) -> Self {
        Orbit {
            polar: 0,
            azimnuth: 0,
            rotation: 0,
            camera_radius,
            focal_length,
            center: Tuple3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }

    pub fn projection(&self) -> Projection {
        let a = rad(self.azimnuth);
        let p = rad(self.polar);
        let r = rad(self.rotation);

        let vec_x = a.sin() * p.cos();
        let vec_y = p.sin();
        let vec_z = a.cos() * p.cos();

        let camera = Tuple3D {
            x: self.center.x + self.camera_radius * vec_x,
            y: self.center.y + self.camera_radius * vec_y,
            z: self.center.z + self.camera_radius * vec_z,
        };

        let screen_radius = self.camera_radius + self.focal_length;
        let screen_coordinate = Tuple3D {
            x: self.center.x + screen_radius * vec_x,
            y: self.center.y + screen_radius * vec_y,
            z: self.center.z + screen_radius * vec_z,
        };

        let u_base = Tuple3D {
            x: a.cos(),
            y: 0.0,
            z: -a.sin(),
        };

        let v_base = Tuple3D {
            x: -a.sin() * p.sin(),
            y: p.cos(),
            z: -a.cos() * p.sin(),
        };

        let cos_r = r.cos();
        let sin_r = r.sin();

        let vector_u = Tuple3D {
            x: cos_r * u_base.x - sin_r * v_base.x,
            y: cos_r * u_base.y - sin_r * v_base.y,
            z: cos_r * u_base.z - sin_r * v_base.z,
        };

        let vector_v = Tuple3D {
            x: sin_r * u_base.x + cos_r * v_base.x,
            y: sin_r * u_base.y + cos_r * v_base.y,
            z: sin_r * u_base.z + cos_r * v_base.z,
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

    pub fn inc_rotation(&mut self) -> Projection {
        if self.rotation == FULL_CIRCLE - 1 {
            self.rotation = 0;
        } else {
            self.rotation += 1;
        }

        self.projection()
    }

    pub fn dec_rotation(&mut self) -> Projection {
        if self.rotation == 0 {
            self.rotation = FULL_CIRCLE - 1;
        } else {
            self.rotation -= 1;
        }

        self.projection()
    }

    pub fn polar(&self) -> u16 {
        self.polar
    }

    pub fn azimuth(&self) -> u16 {
        self.azimnuth
    }

    pub fn rotation(&self) -> u16 {
        self.rotation
    }

    pub fn camera_radius(&self) -> f64 {
        self.camera_radius
    }

    pub fn focal_length(&self) -> f64 {
        self.focal_length
    }

    pub fn center(&self) -> &Tuple3D {
        &self.center
    }

    pub fn set_center(&mut self, center: Tuple3D) {
        self.center = center;
    }

    pub fn reset_to_defaults(&mut self, default_camera_radius: f64, default_focal_length: f64) {
        self.polar = 0;
        self.azimnuth = 0;
        self.rotation = 0;
        self.camera_radius = default_camera_radius;
        self.focal_length = default_focal_length;
        self.center = Tuple3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
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
