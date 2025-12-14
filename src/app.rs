use eframe::egui;

use crate::{camera::Orbit, camera::Projection, Pixel3D, SIZE};

const HALF_SIZE: isize = SIZE as isize / 2;

pub fn map_to_pixels2d(pixels3d: &[Pixel3D], projection: Projection) -> egui::ColorImage {
    let mut pixels2d: Vec<egui::Color32> = vec![egui::Color32::BLACK; SIZE * SIZE];
    let mut distances: Vec<f64> = vec![f64::MAX; SIZE * SIZE];

    for pixel3d in pixels3d {
        let dist_coord_option = projection.project(&pixel3d.coordinate);
        if let Some((distance, coord)) = dist_coord_option {
            let ix = HALF_SIZE + coord.0.round() as isize;
            let iy = HALF_SIZE + coord.1.round() as isize;

            if ix >= 0 && iy >= 0 {
                let x = ix as usize;
                let y = iy as usize;

                if x < SIZE && y < SIZE {
                    let index = y * SIZE + x;
                    if distance < distances[index] {
                        pixels2d[index] = egui::Color32::from_rgb(
                            pixel3d.color.0,
                            pixel3d.color.1,
                            pixel3d.color.2,
                        );
                        distances[index] = distance;
                    }
                }
            }
        }
    }

    egui::ColorImage {
        size: [SIZE, SIZE],
        source_size: egui::Vec2::new(SIZE as f32, SIZE as f32),
        pixels: pixels2d,
    }
}

pub struct PrimeWalkApp {
    pixels: Vec<Pixel3D>,
    orbit: Orbit,
    texture: Option<egui::TextureHandle>,
    default_camera_radius: f64,
    default_focal_length: f64,
}

impl PrimeWalkApp {
    fn new(pixels: Vec<Pixel3D>) -> Self {
        Self {
            pixels,
            orbit: Orbit::new(300.0, 40.0),
            texture: None,
            default_camera_radius: 300.0,
            default_focal_length: 40.0,
        }
    }

    fn update_image(&mut self, ctx: &egui::Context) {
        let projection = self.orbit.projection();
        let color_image = map_to_pixels2d(&self.pixels, projection);

        if let Some(texture) = &mut self.texture {
            texture.set(color_image, egui::TextureOptions::default());
        } else {
            self.texture = Some(ctx.load_texture(
                "prime_walk",
                color_image,
                egui::TextureOptions::default(),
            ));
        }
    }
}

impl eframe::App for PrimeWalkApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut needs_update = false;

        ctx.input(|i| {
            if i.key_down(egui::Key::J) {
                self.orbit.dec_polar();
                needs_update = true;
            }
            if i.key_down(egui::Key::K) {
                self.orbit.inc_polar();
                needs_update = true;
            }
            if i.key_down(egui::Key::H) {
                self.orbit.dec_azimuth();
                needs_update = true;
            }
            if i.key_down(egui::Key::L) {
                self.orbit.inc_azimuth();
                needs_update = true;
            }
            if i.key_down(egui::Key::Z) {
                if i.modifiers.shift {
                    self.orbit.inc_camera_radius();
                } else {
                    self.orbit.dec_camera_radius();
                }
                needs_update = true;
            }
            if i.key_down(egui::Key::F) {
                if i.modifiers.shift {
                    self.orbit.inc_focal_length();
                } else {
                    self.orbit.dec_focal_length();
                }
                needs_update = true;
            }
            if i.key_down(egui::Key::A) {
                let mut center = *self.orbit.center();
                if i.modifiers.shift {
                    center.x += 1.0;
                } else {
                    center.x -= 1.0;
                }
                self.orbit.set_center(center);
                needs_update = true;
            }
            if i.key_down(egui::Key::S) {
                let mut center = *self.orbit.center();
                if i.modifiers.shift {
                    center.y += 1.0;
                } else {
                    center.y -= 1.0;
                }
                self.orbit.set_center(center);
                needs_update = true;
            }
            if i.key_down(egui::Key::W) {
                let mut center = *self.orbit.center();
                if i.modifiers.shift {
                    center.z += 1.0;
                } else {
                    center.z -= 1.0;
                }
                self.orbit.set_center(center);
                needs_update = true;
            }
            if i.key_down(egui::Key::D) {
                self.orbit.reset_to_defaults(
                    self.default_camera_radius,
                    self.default_focal_length,
                );
                needs_update = true;
            }
            if i.key_down(egui::Key::R) {
                if i.modifiers.shift {
                    self.orbit.inc_rotation();
                } else {
                    self.orbit.dec_rotation();
                }
                needs_update = true;
            }
        });

        if needs_update || self.texture.is_none() {
            self.update_image(ctx);
        }

        egui::SidePanel::left("info_panel").show(ctx, |ui| {
            ui.heading("Camera Info");
            ui.separator();
            ui.label(format!("Azimuth: {}", self.orbit.azimuth()));
            ui.label(format!("Polar: {}", self.orbit.polar()));
            ui.label(format!("Rotation: {}", self.orbit.rotation()));
            let center = self.orbit.center();
            ui.label(format!(
                "Center: ({:.1}, {:.1}, {:.1})",
                center.x, center.y, center.z
            ));
            ui.label(format!("Camera radius: {:.1}", self.orbit.camera_radius()));
            ui.label(format!("Focal length: {:.1}", self.orbit.focal_length()));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                egui::Frame::new()
                    .stroke(egui::Stroke::new(2.0, egui::Color32::GREEN))
                    .show(ui, |ui| {
                        ui.image(texture);
                    });
            }
        });

        ctx.request_repaint();
    }
}

pub fn image(pixels: Vec<Pixel3D>) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([SIZE as f32, SIZE as f32])
            .with_title("3D Prime Walk"),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "3D Prime Walk",
        options,
        Box::new(|_cc| Ok(Box::new(PrimeWalkApp::new(pixels)))),
    );
}
