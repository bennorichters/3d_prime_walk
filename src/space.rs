#[derive(Debug, Copy, Clone)]
pub struct Tuple3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Tuple3D {
    pub fn coordinate_squared_distance(&self, other: &Tuple3D) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;

        dx * dx + dy * dy + dz * dz
    }

    fn dot(&self, other: &Tuple3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: &Tuple3D) -> Tuple3D {
        Tuple3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn sub(&self, other: &Tuple3D) -> Tuple3D {
        Tuple3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn scale(&self, t: f64) -> Tuple3D {
        Tuple3D {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }

    fn add(&self, other: &Tuple3D) -> Tuple3D {
        Tuple3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

pub struct Pixel3D {
    pub coordinate: Tuple3D,
    pub color: (u8, u8, u8),
}

#[derive(Debug)]
pub struct Screen {
    pub coordinate: Tuple3D,
    pub vector_u: Tuple3D,
    pub vector_v: Tuple3D,
    pub width: usize,
    pub height: usize,
}

impl Screen {
    pub fn intersect(&self, camera: &Tuple3D, target: &Tuple3D) -> Option<(usize, usize)> {
        let n = self.vector_u.cross(&self.vector_v);

        let dist1 = camera.sub(&self.coordinate).dot(&n);
        let dist2 = target.sub(&self.coordinate).dot(&n);
        if dist1 * dist2 <= 0.0 {
            return None;
        }

        let d = target.sub(camera);
        let denom = d.dot(&n);
        let t = -dist1 / denom;
        let q = camera.add(&d.scale(t));

        let diff = q.sub(&self.coordinate);
        let u = diff.dot(&self.vector_u) / self.vector_u.dot(&self.vector_u);
        let v = diff.dot(&self.vector_v) / self.vector_v.dot(&self.vector_v);

        // Convert to pixel coordinates (coordinate is the center of the plane)
        let half_width = self.width as f64 / 2.0;
        let half_height = self.height as f64 / 2.0;

        let pixel_x = (u + half_width).round();
        let pixel_y = (v + half_height).round();

        // Check if within boundaries (after rounding)
        if pixel_x < 0.0 || pixel_x >= self.width as f64 ||
           pixel_y < 0.0 || pixel_y >= self.height as f64 {
            return None;
        }

        Some((pixel_x as usize, pixel_y as usize))
    }

    pub fn corners(&self) -> [Tuple3D; 4] {
        let half_width = self.width as f64 / 2.0;
        let half_height = self.height as f64 / 2.0;

        let top_left = self.coordinate
            .add(&self.vector_u.scale(-half_width))
            .add(&self.vector_v.scale(-half_height));

        let top_right = self.coordinate
            .add(&self.vector_u.scale(half_width))
            .add(&self.vector_v.scale(-half_height));

        let bottom_left = self.coordinate
            .add(&self.vector_u.scale(-half_width))
            .add(&self.vector_v.scale(half_height));

        let bottom_right = self.coordinate
            .add(&self.vector_u.scale(half_width))
            .add(&self.vector_v.scale(half_height));

        [top_left, top_right, bottom_left, bottom_right]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel() {
        let p = Screen {
            coordinate: Tuple3D {
                x: 0.0,
                y: 260.0,
                z: 0.0,
            },
            vector_u: Tuple3D {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            vector_v: Tuple3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            width: 100,
            height: 100,
        };

        let c1 = Tuple3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let c2 = Tuple3D {
            x: 0.0,
            y: 280.0,
            z: 0.0,
        };

        let a = p.intersect(&c1, &c2);
        assert!(a.is_none());
    }
}
