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

#[derive(Debug)]
pub struct Plane {
    pub coordinate: Tuple3D,
    pub vector_u: Tuple3D,
    pub vector_v: Tuple3D,
}

impl Plane {
    pub fn intersect(&self, coord1: &Tuple3D, coord2: &Tuple3D) -> Option<(f64, f64)> {
        let n = self.vector_u.cross(&self.vector_v);

        let dist1 = coord1.sub(&self.coordinate).dot(&n);
        let dist2 = coord2.sub(&self.coordinate).dot(&n);
        if dist1 * dist2 >= 0.0 {
            return None;
        }

        let d = coord2.sub(coord1);
        let denom = d.dot(&n);
        let t = -dist1 / denom;
        let q = coord1.add(&d.scale(t));

        let diff = q.sub(&self.coordinate);
        let u = diff.dot(&self.vector_u) / self.vector_u.dot(&self.vector_u);
        let v = diff.dot(&self.vector_v) / self.vector_v.dot(&self.vector_v);

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
