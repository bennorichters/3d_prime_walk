use crate::space::{Pixel3D, Plane, Screen, Tuple3D};
use eframe::egui;
use std::f64::consts::PI;

use crate::SIZE;

const FULL_CIRCLE: u16 = 360;
const HALF_CIRCLE: u16 = 180;

fn rad(angle: u16) -> f64 {
    (angle as f64 * PI) / HALF_CIRCLE as f64
}

pub struct Projection {
    camera: Tuple3D,
    screen: Screen,
    planes: [Plane; 4],
}

impl Projection {
    fn new(camera: Tuple3D, screen: Screen) -> Self {
        let [top_left, top_right, bottom_left, bottom_right] = screen.corners;

        let planes = [
            Plane::new(top_left, top_right, camera),
            Plane::new(top_right, bottom_right, camera),
            Plane::new(bottom_left, bottom_right, camera),
            Plane::new(bottom_left, top_left, camera),
        ];

        Self {
            camera,
            screen,
            planes,
        }
    }

    fn draw_line(
        &self,
        from: (usize, usize),
        to: (usize, usize),
        color: egui::Color32,
        distance: f64,
        pixels2d: &mut [egui::Color32],
        distances: &mut [f64],
    ) {
        let (x0, y0) = (from.0 as isize, from.1 as isize);
        let (x1, y1) = (to.0 as isize, to.1 as isize);

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            if x >= 0 && x < SIZE as isize && y >= 0 && y < SIZE as isize {
                let index = (y as usize) * SIZE + (x as usize);
                if distance < distances[index] {
                    pixels2d[index] = color;
                    distances[index] = distance;
                }
            }

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn pixel_color(pixel3d: &Pixel3D) -> egui::Color32 {
        egui::Color32::from_rgb(pixel3d.color.0, pixel3d.color.1, pixel3d.color.2)
    }

    fn draw_line_from_prev(
        &self,
        prev_coord: Option<(f64, (usize, usize))>,
        current_pos: (usize, usize),
        color: egui::Color32,
        distance: f64,
        pixels2d: &mut [egui::Color32],
        distances: &mut [f64],
    ) {
        if let Some((_, prev_xy)) = prev_coord {
            self.draw_line(prev_xy, current_pos, color, distance, pixels2d, distances);
        }
    }

    fn handle_projected_point(
        &self,
        coord_3d: &Tuple3D,
        pixel3d: &Pixel3D,
        prev_coord: Option<(f64, (usize, usize))>,
        pixels2d: &mut [egui::Color32],
        distances: &mut [f64],
    ) -> Option<(f64, (usize, usize))> {
        self.screen
            .project(&self.camera, coord_3d)
            .map(|relative_coords| {
                let distance = self.camera.coordinate_squared_distance(coord_3d);
                let color = Self::pixel_color(pixel3d);

                self.draw_line_from_prev(
                    prev_coord,
                    relative_coords,
                    color,
                    distance,
                    pixels2d,
                    distances,
                );

                (distance, relative_coords)
            })
    }

    fn handle_edge_intersection(
        &self,
        prev_3d: &Tuple3D,
        current_3d: &Tuple3D,
        pixel3d: &Pixel3D,
        prev_coord: Option<(f64, (usize, usize))>,
        pixels2d: &mut [egui::Color32],
        distances: &mut [f64],
    ) {
        let edge_results = self.edge(prev_3d, current_3d);

        if let Some(intersection) = edge_results.iter().find_map(|&opt| opt) {
            self.handle_projected_point(
                &intersection,
                pixel3d,
                prev_coord,
                pixels2d,
                distances,
            );
        }
    }

    pub fn map_to_pixels2d(&self, pixels3d: &[Pixel3D]) -> egui::ColorImage {
        let mut pixels2d: Vec<egui::Color32> = vec![egui::Color32::BLACK; SIZE * SIZE];
        let mut distances: Vec<f64> = vec![f64::MAX; SIZE * SIZE];

        let mut prev_coord: Option<(f64, (usize, usize))> = None;
        let mut prev_3d_coord: Option<Tuple3D> = None;

        for pixel3d in pixels3d {
            let projected = self.handle_projected_point(
                &pixel3d.coordinate,
                pixel3d,
                prev_coord,
                &mut pixels2d,
                &mut distances,
            );

            if projected.is_some() {
                prev_coord = projected;
                prev_3d_coord = Some(pixel3d.coordinate);
            } else {
                if let Some(prev_3d) = prev_3d_coord {
                    self.handle_edge_intersection(
                        &prev_3d,
                        &pixel3d.coordinate,
                        pixel3d,
                        prev_coord,
                        &mut pixels2d,
                        &mut distances,
                    );
                }

                prev_coord = None;
                prev_3d_coord = Some(pixel3d.coordinate);
            }
        }

        egui::ColorImage {
            size: [SIZE, SIZE],
            source_size: egui::Vec2::new(SIZE as f32, SIZE as f32),
            pixels: pixels2d,
        }
    }

    pub fn edge(&self, start: &Tuple3D, end: &Tuple3D) -> [Option<Tuple3D>; 4] {
        [
            self.planes[0].intersect(start, end),
            self.planes[1].intersect(start, end),
            self.planes[2].intersect(start, end),
            self.planes[3].intersect(start, end),
        ]
    }
}

pub struct Orbit {
    polar: u16,
    azimnuth: u16,
    rotation: u16,
    camera_radius: f64,
    focal_length: f64,
    center: Tuple3D,
    screen_width: usize,
    screen_height: usize,
}

impl Orbit {
    pub fn new(
        camera_radius: f64,
        focal_length: f64,
        screen_width: usize,
        screen_height: usize,
    ) -> Self {
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
            screen_width,
            screen_height,
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

        Projection::new(
            camera,
            Screen::new(
                screen_coordinate,
                vector_u,
                vector_v,
                self.screen_width,
                self.screen_height,
            ),
        )
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
        if self.camera_radius >= 1.0 {
            self.camera_radius -= 1.0;
        }
        self.projection()
    }

    pub fn inc_focal_length(&mut self) -> Projection {
        self.focal_length += 1.0;
        self.projection()
    }

    pub fn dec_focal_length(&mut self) -> Projection {
        if self.focal_length > 1.0 {
            self.focal_length -= 1.0;
        }
        self.projection()
    }
}
