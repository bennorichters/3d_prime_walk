use eframe::egui;

use crate::{camera::Orbit, map_to_pixels2d, Pixel3D, SIZE};

pub struct PrimeWalkApp {
    pixels: Vec<Pixel3D>,
    orbit: Orbit,
    texture: Option<egui::TextureHandle>,
}

impl PrimeWalkApp {
    fn new(pixels: Vec<Pixel3D>) -> Self {
        Self {
            pixels,
            orbit: Orbit::new(),
            texture: None,
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
        });

        if needs_update || self.texture.is_none() {
            self.update_image(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                ui.image(texture);
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
