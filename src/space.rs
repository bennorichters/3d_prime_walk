#[derive(Clone, Copy, Debug)]
pub struct Tuple3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Tuple3D {
    pub fn coordinate_distance(&self, other: &Tuple3D) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;

        // TODO: sqrt not necessary for purpose
        (dx * dx + dy * dy + dz * dz).sqrt()
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

#[derive(Debug)]
pub struct Plane {
    pub coordinate: Tuple3D,
    pub vector1: Tuple3D,
    pub vector2: Tuple3D,
}

impl Plane {
    pub fn intersect(&self, coord1: &Tuple3D, coord2: &Tuple3D) -> Option<(f64, f64)> {
        let n = self.vector1.cross(&self.vector2); // TODO: only once
        let d = coord2.sub(coord1);

        let denom = d.dot(&n);
        if denom.abs() < 1e-10 {
            return None; // parallel
        }

        let t = self.coordinate.sub(coord1).dot(&n) / denom;
        let q = coord1.add(&d.scale(t));
        let diff = q.sub(&self.coordinate);

        let u = diff.dot(&self.vector1) / self.vector1.dot(&self.vector1);
        let v = diff.dot(&self.vector2) / self.vector2.dot(&self.vector2);

        Some((u, v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aap() {
        let p = Plane {
            coordinate: Tuple3D {
                x: 0.0,
                y: 260.0,
                z: 0.0,
            },
            vector1: Tuple3D {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            vector2: Tuple3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
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
