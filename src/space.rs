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

    pub fn dot(&self, other: &Tuple3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Tuple3D) -> Tuple3D {
        Tuple3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn sub(&self, other: &Tuple3D) -> Tuple3D {
        Tuple3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn scale(&self, t: f64) -> Tuple3D {
        Tuple3D {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }

    pub fn add(&self, other: &Tuple3D) -> Tuple3D {
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

pub struct Plane {
    point1: Tuple3D,
    normal: Tuple3D,
}

impl Plane {
    pub fn new(point1: Tuple3D, point2: Tuple3D, point3: Tuple3D) -> Self {
        let v1 = point2.sub(&point1);
        let v2 = point3.sub(&point1);

        let normal = v1.cross(&v2);

        Self { point1, normal }
    }

    pub fn intersect(&self, start: &Tuple3D, end: &Tuple3D) -> Option<Tuple3D> {
        let d = end.sub(start);

        let denom = self.normal.dot(&d);
        if denom.abs() < 1e-10 {
            return None;
        }

        let diff = self.point1.sub(start);
        let t = self.normal.dot(&diff) / denom;

        if !(0.0..=1.0).contains(&t) {
            return None;
        }

        Some(start.add(&d.scale(t)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane_intersect_normal_case() {
        let plane = Plane::new(
            Tuple3D {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            Tuple3D {
                x: 1.0,
                y: 0.0,
                z: 5.0,
            },
            Tuple3D {
                x: 0.0,
                y: 1.0,
                z: 5.0,
            },
        );

        let start = Tuple3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let end = Tuple3D {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        };

        let result = plane.intersect(&start, &end);
        assert!(result.is_some());
        let point = result.unwrap();
        assert!((point.x - 0.0).abs() < 1e-10);
        assert!((point.y - 0.0).abs() < 1e-10);
        assert!((point.z - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_plane_intersect_parallel_line() {
        let plane = Plane::new(
            Tuple3D {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            Tuple3D {
                x: 1.0,
                y: 0.0,
                z: 5.0,
            },
            Tuple3D {
                x: 0.0,
                y: 1.0,
                z: 5.0,
            },
        );

        let start = Tuple3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let end = Tuple3D {
            x: 10.0,
            y: 10.0,
            z: 0.0,
        };

        let result = plane.intersect(&start, &end);
        assert!(result.is_none());
    }

    #[test]
    fn test_plane_intersect_segment_too_short() {
        let plane = Plane::new(
            Tuple3D {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            Tuple3D {
                x: 1.0,
                y: 0.0,
                z: 5.0,
            },
            Tuple3D {
                x: 0.0,
                y: 1.0,
                z: 5.0,
            },
        );

        let start = Tuple3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let end = Tuple3D {
            x: 0.0,
            y: 0.0,
            z: 3.0,
        };

        let result = plane.intersect(&start, &end);
        assert!(result.is_none());
    }

    #[test]
    fn test_plane_intersect_segment_past_plane() {
        let plane = Plane::new(
            Tuple3D {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            Tuple3D {
                x: 1.0,
                y: 0.0,
                z: 5.0,
            },
            Tuple3D {
                x: 0.0,
                y: 1.0,
                z: 5.0,
            },
        );

        let start = Tuple3D {
            x: 0.0,
            y: 0.0,
            z: 6.0,
        };
        let end = Tuple3D {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        };

        let result = plane.intersect(&start, &end);
        assert!(result.is_none());
    }

    #[test]
    fn test_plane_intersect_at_start_point() {
        let plane = Plane::new(
            Tuple3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Tuple3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Tuple3D {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        );

        let start = Tuple3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let end = Tuple3D {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        };

        let result = plane.intersect(&start, &end);
        assert!(result.is_some());
        let point = result.unwrap();
        assert!((point.x - 0.0).abs() < 1e-10);
        assert!((point.y - 0.0).abs() < 1e-10);
        assert!((point.z - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_plane_intersect_at_end_point() {
        let plane = Plane::new(
            Tuple3D {
                x: 0.0,
                y: 0.0,
                z: 10.0,
            },
            Tuple3D {
                x: 1.0,
                y: 0.0,
                z: 10.0,
            },
            Tuple3D {
                x: 0.0,
                y: 1.0,
                z: 10.0,
            },
        );

        let start = Tuple3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let end = Tuple3D {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        };

        let result = plane.intersect(&start, &end);
        assert!(result.is_some());
        let point = result.unwrap();
        assert!((point.x - 0.0).abs() < 1e-10);
        assert!((point.y - 0.0).abs() < 1e-10);
        assert!((point.z - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_plane_intersect_angled_plane() {
        let plane = Plane::new(
            Tuple3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Tuple3D {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
            Tuple3D {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        );

        let start = Tuple3D {
            x: 0.5,
            y: 0.5,
            z: -1.0,
        };
        let end = Tuple3D {
            x: 0.5,
            y: 0.5,
            z: 2.0,
        };

        let result = plane.intersect(&start, &end);
        assert!(result.is_some());
        let point = result.unwrap();
        assert!((point.x - 0.5).abs() < 1e-10);
        assert!((point.y - 0.5).abs() < 1e-10);
        assert!((point.z - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_plane_intersect_segment_in_plane() {
        let plane = Plane::new(
            Tuple3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Tuple3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Tuple3D {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        );

        let start = Tuple3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let end = Tuple3D {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        };

        let result = plane.intersect(&start, &end);
        assert!(result.is_none());
    }
}
