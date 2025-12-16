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
        // Calculate two vectors in the plane
        let v1 = point2.sub(&point1);
        let v2 = point3.sub(&point1);

        // Calculate plane normal
        let normal = v1.cross(&v2);

        Self { point1, normal }
    }

    pub fn intersect(&self, start: &Tuple3D, end: &Tuple3D) -> Option<Tuple3D> {
        // Line segment direction
        let d = end.sub(start);

        // Check if line is parallel to plane
        let denom = self.normal.dot(&d);
        if denom.abs() < 1e-10 {
            return None;
        }

        // Calculate parameter t
        let diff = self.point1.sub(start);
        let t = self.normal.dot(&diff) / denom;

        // Check if intersection is within segment bounds
        if !(0.0..=1.0).contains(&t) {
            return None;
        }

        // Calculate intersection point
        Some(start.add(&d.scale(t)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane_intersect_normal_case() {
        // Plane parallel to XY plane at z = 5
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

        // Line segment from (0, 0, 0) to (0, 0, 10) should intersect at (0, 0, 5)
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
        // Plane parallel to XY plane at z = 5
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

        // Line segment parallel to the plane (in XY plane at z = 0)
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
        // Plane parallel to XY plane at z = 5
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

        // Line segment doesn't reach the plane
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
        // Plane parallel to XY plane at z = 5
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

        // Line segment starts after the plane
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
        // Plane parallel to XY plane at z = 0
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

        // Line segment starts on the plane
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
        // Plane parallel to XY plane at z = 10
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

        // Line segment ends on the plane
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
        // Plane defined by three points forming a diagonal plane
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

        // Line segment from (0.5, 0.5, -1) to (0.5, 0.5, 2)
        // Should intersect at (0.5, 0.5, 0.5)
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
        // Plane parallel to XY plane at z = 0
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

        // Line segment lies entirely in the plane
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
        // Should return None because the line is parallel (lies in the plane)
        assert!(result.is_none());
    }
}
