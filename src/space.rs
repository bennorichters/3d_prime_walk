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
    coordinate: Tuple3D,
    vector_u: Tuple3D,
    vector_v: Tuple3D,
    width: usize,
    height: usize,
    corners: [Tuple3D; 4],
}

impl Screen {
    pub fn new(coordinate: Tuple3D, vector_u: Tuple3D, vector_v: Tuple3D, width: usize, height: usize) -> Self {
        let half_width = width as f64 / 2.0;
        let half_height = height as f64 / 2.0;

        let top_left = coordinate
            .add(&vector_u.scale(-half_width))
            .add(&vector_v.scale(-half_height));

        let top_right = coordinate
            .add(&vector_u.scale(half_width))
            .add(&vector_v.scale(-half_height));

        let bottom_left = coordinate
            .add(&vector_u.scale(-half_width))
            .add(&vector_v.scale(half_height));

        let bottom_right = coordinate
            .add(&vector_u.scale(half_width))
            .add(&vector_v.scale(half_height));

        Screen {
            coordinate,
            vector_u,
            vector_v,
            width,
            height,
            corners: [top_left, top_right, bottom_left, bottom_right],
        }
    }

    pub fn project(&self, camera: &Tuple3D, target: &Tuple3D) -> Option<(usize, usize)> {
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

    pub fn edge(&self, start: &Tuple3D, end: &Tuple3D) -> [Option<Tuple3D>; 4] {
        let [top_left, top_right, bottom_left, bottom_right] = self.corners;

        let planes = [
            Plane {
                point1: top_left,
                point2: top_right,
                point3: self.coordinate,
            },
            Plane {
                point1: top_right,
                point2: bottom_right,
                point3: self.coordinate,
            },
            Plane {
                point1: bottom_left,
                point2: bottom_right,
                point3: self.coordinate,
            },
            Plane {
                point1: bottom_left,
                point2: top_left,
                point3: self.coordinate,
            },
        ];

        [
            planes[0].intersect(start, end),
            planes[1].intersect(start, end),
            planes[2].intersect(start, end),
            planes[3].intersect(start, end),
        ]
    }
}

pub struct Plane {
    pub point1: Tuple3D,
    pub point2: Tuple3D,
    pub point3: Tuple3D,
}

impl Plane {
    pub fn intersect(&self, start: &Tuple3D, end: &Tuple3D) -> Option<Tuple3D> {
        // Calculate two vectors in the plane
        let v1 = self.point2.sub(&self.point1);
        let v2 = self.point3.sub(&self.point1);

        // Calculate plane normal
        let n = v1.cross(&v2);

        // Line segment direction
        let d = end.sub(start);

        // Check if line is parallel to plane
        let denom = n.dot(&d);
        if denom.abs() < 1e-10 {
            return None;
        }

        // Calculate parameter t
        let diff = self.point1.sub(start);
        let t = n.dot(&diff) / denom;

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
    fn test_parallel() {
        let p = Screen::new(
            Tuple3D {
                x: 0.0,
                y: 260.0,
                z: 0.0,
            },
            Tuple3D {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            Tuple3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            100,
            100,
        );

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

        let a = p.project(&c1, &c2);
        assert!(a.is_none());
    }

    #[test]
    fn test_plane_intersect_normal_case() {
        // Plane parallel to XY plane at z = 5
        let plane = Plane {
            point1: Tuple3D { x: 0.0, y: 0.0, z: 5.0 },
            point2: Tuple3D { x: 1.0, y: 0.0, z: 5.0 },
            point3: Tuple3D { x: 0.0, y: 1.0, z: 5.0 },
        };

        // Line segment from (0, 0, 0) to (0, 0, 10) should intersect at (0, 0, 5)
        let start = Tuple3D { x: 0.0, y: 0.0, z: 0.0 };
        let end = Tuple3D { x: 0.0, y: 0.0, z: 10.0 };

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
        let plane = Plane {
            point1: Tuple3D { x: 0.0, y: 0.0, z: 5.0 },
            point2: Tuple3D { x: 1.0, y: 0.0, z: 5.0 },
            point3: Tuple3D { x: 0.0, y: 1.0, z: 5.0 },
        };

        // Line segment parallel to the plane (in XY plane at z = 0)
        let start = Tuple3D { x: 0.0, y: 0.0, z: 0.0 };
        let end = Tuple3D { x: 10.0, y: 10.0, z: 0.0 };

        let result = plane.intersect(&start, &end);
        assert!(result.is_none());
    }

    #[test]
    fn test_plane_intersect_segment_too_short() {
        // Plane parallel to XY plane at z = 5
        let plane = Plane {
            point1: Tuple3D { x: 0.0, y: 0.0, z: 5.0 },
            point2: Tuple3D { x: 1.0, y: 0.0, z: 5.0 },
            point3: Tuple3D { x: 0.0, y: 1.0, z: 5.0 },
        };

        // Line segment doesn't reach the plane
        let start = Tuple3D { x: 0.0, y: 0.0, z: 0.0 };
        let end = Tuple3D { x: 0.0, y: 0.0, z: 3.0 };

        let result = plane.intersect(&start, &end);
        assert!(result.is_none());
    }

    #[test]
    fn test_plane_intersect_segment_past_plane() {
        // Plane parallel to XY plane at z = 5
        let plane = Plane {
            point1: Tuple3D { x: 0.0, y: 0.0, z: 5.0 },
            point2: Tuple3D { x: 1.0, y: 0.0, z: 5.0 },
            point3: Tuple3D { x: 0.0, y: 1.0, z: 5.0 },
        };

        // Line segment starts after the plane
        let start = Tuple3D { x: 0.0, y: 0.0, z: 6.0 };
        let end = Tuple3D { x: 0.0, y: 0.0, z: 10.0 };

        let result = plane.intersect(&start, &end);
        assert!(result.is_none());
    }

    #[test]
    fn test_plane_intersect_at_start_point() {
        // Plane parallel to XY plane at z = 0
        let plane = Plane {
            point1: Tuple3D { x: 0.0, y: 0.0, z: 0.0 },
            point2: Tuple3D { x: 1.0, y: 0.0, z: 0.0 },
            point3: Tuple3D { x: 0.0, y: 1.0, z: 0.0 },
        };

        // Line segment starts on the plane
        let start = Tuple3D { x: 0.0, y: 0.0, z: 0.0 };
        let end = Tuple3D { x: 0.0, y: 0.0, z: 10.0 };

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
        let plane = Plane {
            point1: Tuple3D { x: 0.0, y: 0.0, z: 10.0 },
            point2: Tuple3D { x: 1.0, y: 0.0, z: 10.0 },
            point3: Tuple3D { x: 0.0, y: 1.0, z: 10.0 },
        };

        // Line segment ends on the plane
        let start = Tuple3D { x: 0.0, y: 0.0, z: 0.0 };
        let end = Tuple3D { x: 0.0, y: 0.0, z: 10.0 };

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
        let plane = Plane {
            point1: Tuple3D { x: 0.0, y: 0.0, z: 0.0 },
            point2: Tuple3D { x: 1.0, y: 0.0, z: 1.0 },
            point3: Tuple3D { x: 0.0, y: 1.0, z: 0.0 },
        };

        // Line segment from (0.5, 0.5, -1) to (0.5, 0.5, 2)
        // Should intersect at (0.5, 0.5, 0.5)
        let start = Tuple3D { x: 0.5, y: 0.5, z: -1.0 };
        let end = Tuple3D { x: 0.5, y: 0.5, z: 2.0 };

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
        let plane = Plane {
            point1: Tuple3D { x: 0.0, y: 0.0, z: 0.0 },
            point2: Tuple3D { x: 1.0, y: 0.0, z: 0.0 },
            point3: Tuple3D { x: 0.0, y: 1.0, z: 0.0 },
        };

        // Line segment lies entirely in the plane
        let start = Tuple3D { x: 0.0, y: 0.0, z: 0.0 };
        let end = Tuple3D { x: 1.0, y: 1.0, z: 0.0 };

        let result = plane.intersect(&start, &end);
        // Should return None because the line is parallel (lies in the plane)
        assert!(result.is_none());
    }
}
