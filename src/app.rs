use eframe::egui;

use crate::{camera::Orbit, space::Pixel3D, SIZE};

pub struct PrimeWalkApp {
    pixels: Vec<Pixel3D>,
    orbit: Orbit,
    texture: Option<egui::TextureHandle>,
    default_camera_radius: f64,
    default_focal_length: f64,
}

impl PrimeWalkApp {
    fn new(pixels: Vec<Pixel3D>, default_camera_radius: f64, default_focal_length: f64) -> Self {
        Self {
            pixels,
            orbit: Orbit::new(default_camera_radius, default_focal_length, SIZE, SIZE),
            texture: None,
            default_camera_radius,
            default_focal_length,
        }
    }

    fn update_image(&mut self, ctx: &egui::Context) {
        let mut projection = self.orbit.projection();
        let color_image = projection.map_to_pixels2d(&self.pixels);

        if let Some(texture) = &mut self.texture {
            texture.set(color_image, egui::TextureOptions::default());
        } else {
            self.texture =
                Some(ctx.load_texture("prime_walk", color_image, egui::TextureOptions::default()));
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
                let center = *self.orbit.center();
                let u_vec = self.orbit.get_u_vector();
                let offset = if i.modifiers.shift { 1.0 } else { -1.0 };
                let new_center = center.add(&u_vec.scale(offset));
                self.orbit.set_center(new_center);
                needs_update = true;
            }
            if i.key_down(egui::Key::S) {
                let center = *self.orbit.center();
                let v_vec = self.orbit.get_v_vector();
                let offset = if i.modifiers.shift { 1.0 } else { -1.0 };
                let new_center = center.add(&v_vec.scale(offset));
                self.orbit.set_center(new_center);
                needs_update = true;
            }
            if i.key_down(egui::Key::W) {
                let center = *self.orbit.center();
                let normal_vec = self.orbit.get_normal_vector();
                let offset = if i.modifiers.shift { 1.0 } else { -1.0 };
                let new_center = center.add(&normal_vec.scale(offset));
                self.orbit.set_center(new_center);
                needs_update = true;
            }
            if i.key_down(egui::Key::D) {
                self.orbit
                    .reset_to_defaults(self.default_camera_radius, self.default_focal_length);
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

            ui.add_space(10.0);
            ui.heading("Keyboard Controls");
            ui.separator();

            ui.label("Rotation:");
            ui.label("  H/L - Azimuth");
            ui.label("  J/K - Polar");
            ui.label("  R/Shift+R - Rotation");

            ui.add_space(5.0);
            ui.label("Camera:");
            ui.label("  Z/Shift+Z - Distance");
            ui.label("  F/Shift+F - Focal Length");

            ui.add_space(5.0);
            ui.label("Center Position:");
            ui.label("  A/Shift+A - Screen horizontal (vector_u)");
            ui.label("  S/Shift+S - Screen vertical (vector_v)");
            ui.label("  W/Shift+W - Screen normal (toward/away)");

            ui.add_space(5.0);
            ui.label("D - Reset to defaults");
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

        if needs_update {
            ctx.request_repaint();
        }
    }
}

pub fn image(pixels: Vec<Pixel3D>, default_camera_radius: f64, default_focal_length: f64) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([SIZE as f32, SIZE as f32])
            .with_title("3D Prime Walk"),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "3D Prime Walk",
        options,
        Box::new(|_cc| Ok(Box::new(PrimeWalkApp::new(pixels, default_camera_radius, default_focal_length)))),
    );
}
